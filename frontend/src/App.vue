<script setup lang="ts">
import {onMounted, ref} from 'vue'
import axios from 'axios'

let channels = ref([])
let messages = ref({})
let activeChannelId = ref('')
let inputMessage = ref('')
let newChannelName = ref('')

onMounted(() => {
  loadChannels()
  setInterval(() => {
    loadChannelContent(activeChannelId.value)
  }, 5000)
})

function loadChannels(){
  axios
      .get('http://localhost:8080/api/v1/channels/')
      .then(response => {
        channels.value = response.data
        activeChannelId.value = channels.value[0]?.id ?? ''
        loadChannelContent(activeChannelId.value)
      })
}

function loadChannelContent(channelId: string) {
  if (channelId === '') {
    return
  }
  activeChannelId.value = channelId
  console.log(activeChannelId.value)
  axios
      .get(`http://localhost:8080/api/v1/channels/${channelId}/messages/`)
      .then(response => {
        messages.value = response.data
        console.log(messages.value)
      })
}

function sendMessage() {
  if (inputMessage.value === '') {
    return
  }
  axios
      .post(`http://localhost:8080/api/v1/channels/${activeChannelId.value}/messages/`, {
      })
      .then(response => {
        console.log(response.data)
        let message = response.data
        message.content = inputMessage.value
        axios.put(`http://localhost:8080/api/v1/channels/${activeChannelId.value}/messages/${response.data.id}`, message).then(response => {
          inputMessage.value = ''
          loadChannelContent(activeChannelId.value)
        })
      })
}

function createNewChannel() {
  if (newChannelName.value === '') {
    return
  }
  axios
      .post('http://localhost:8080/api/v1/channels/', )
      .then(response => {
        console.log(response.data)
        let channel = response.data
        channel.name = newChannelName.value
        axios.put(`http://localhost:8080/api/v1/channels/${response.data.id}`, channel).then(response => {
          newChannelName.value = ''
          loadChannels()
          loadChannelContent(response.data.id)
        })
      })
}

</script>

<template>
  <div class="app">
    <div class="channel-navigator">
      <div class="channel-navigator-content">
        <ul class="channel-list">
          <li class="channel-entry" v-if="channels !== undefined" v-for="channel in channels" :key="channel.id">
            <button @click="loadChannelContent(channel.id)">
              <p class="channel-entry__name" contenteditable="false">{{ channel.name }}</p>
            </button>
          </li>
          <li class="channel-create-entry">
            <input type="text" v-model="newChannelName" />
            <button @click="createNewChannel">+</button>
          </li>
        </ul>
      </div>
    </div>
    <div class="messages">
      <div class="channel-name-container">
        <h4 class="channel-name" v-if="channels != undefined">
          {{ channels?.find(channel => channel.id === activeChannelId)?.name }}
        </h4>
      </div>
      <div class="message-container">
        <div class="message" v-for="message in messages" :key="message.id">
          <p>{{ message.content }}</p>
        </div>
      </div>
      <div class="send-container">
        <input type="text" v-model="inputMessage" placeholder="Type your message here" />
        <button @click="sendMessage" class="send-button">Send</button>
      </div>
    </div>
  </div>
</template>

<style scoped>

</style>
