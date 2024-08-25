import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  vus: 50,
  duration: '15s',
};

export default function() {
  // GET Request
  http.get(`${__ENV.BASE_API_URL}channels/`)

  // POST Request
  const response = http.post(`${__ENV.BASE_API_URL}channels/`);
  if (response.status !== 201) {
    console.log(response.status)
    console.log("Post failed... skipping!");
    console.log(response.body)
    sleep(0.5);
    return
  }

  // PUT Request
  const body = JSON.parse(response.body);
  const id = body.id;
  body.description = "k6 Automated Test";

  console.log(id)

  const params ={
    headers: {
      'Content-Type': 'application/json'
    }
  }
  const request_body = {
    "createdAt" : body.createdAt,
    "description" : body.description,
    "isPublic" : body.isPublic,
    "id" : body.id,
    "members" : [],
    "messages" : [],
    "name" : body.name,
    "ownerId" : body.ownerId,
    "updatedAt" : body.updatedAt,
  }
  const response_put = http.put(`${__ENV.BASE_API_URL}channels/${id}`, JSON.stringify(request_body) ,params);
  if (response_put.status !== 200){
    console.log(response_put.status)
    console.log("Put failed... skipping!")
    console.log(response_put.body)
    sleep(2)
    return
  }

  // GET Request
  const response_get = http.get(`${__ENV.BASE_API_URL}channels/${id}`);
  if (response_get.status !== 200){
    console.log(response_get.status)
    console.log(`GET failed for id ${id}... skipping!`)
    console.log(response_get.body)
  }

  // DELETE Request
  const response_del = http.del(`${__ENV.BASE_API_URL}channels/${id}`);
  if (response_del.status !== 200){
    console.log(response_del.status)
    console.log("DELETE failed... skipping!")
    console.log(response_del.body)
  }
  sleep(0.5);
}

export function handleSummary(data){
    let label = `${__ENV.TEST_ENV}-profiles-channels.json`;
    let results = {};
    results[label] = JSON.stringify(data);
    return results;
}

