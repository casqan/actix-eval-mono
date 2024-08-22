import http from 'k6/http';
import {sleep} from 'k6';

export const options = {
    // A number specifying the number of VUs to run concurrently.
    vus: 10,
    // A string specifying the total duration of the test run.
    duration: '30s',

    // The following section contains configuration options for execution of this
    // test script in Grafana Cloud.
    //
    // See https://grafana.com/docs/grafana-cloud/k6/get-started/run-cloud-tests-from-the-cli/
    // to learn about authoring and running k6 test scripts in Grafana k6 Cloud.
    //
    // cloud: {
    //   // The ID of the project to which the test is assigned in the k6 Cloud UI.
    //   // By default tests are executed in default project.
    //   projectID: "",
    //   // The name of the test in the k6 Cloud UI.
    //   // Test runs with the same name will be grouped.
    //   name: "script.js"
    // },

    // Uncomment this section to enable the use of Browser API in your tests.
    //
    // See https://grafana.com/docs/k6/latest/using-k6-browser/running-browser-tests/ to learn more
    // about using Browser API in your test scripts.
    //
    // scenarios: {
    //   // The scenario name appears in the result summary, tags, and so on.
    //   // You can give the scenario any name, as long as each name in the script is unique.
    //   ui: {
    //     // Executor is a mandatory parameter for browser-based tests.
    //     // Shared iterations in this case tells k6 to reuse VUs to execute iterations.
    //     //
    //     // See https://grafana.com/docs/k6/latest/using-k6/scenarios/executors/ for other executor types.
    //     executor: 'shared-iterations',
    //     options: {
    //       browser: {
    //         // This is a mandatory parameter that instructs k6 to launch and
    //         // connect to a chromium-based browser, and use it to run UI-based
    //         // tests.
    //         type: 'chromium',
    //       },
    //     },
    //   },
    // }
};

const MESSAGES_PER_ITERATION = 20

export default function () {
    // POST Request
    const post_channel_res = http.post(`${__ENV.BASE_API_URL}channels/`);
    if (post_channel_res.status !== 201) {
        console.log(post_channel_res.status)
        console.log("Post failed... skipping!");
        console.log(post_channel_res.body)
        sleep(0.5);
        return
    }
    const body = JSON.parse(post_channel_res.body);
    const channel_id = body.id;

    for (let i = 0; i < MESSAGES_PER_ITERATION; i++) {
        //POST new message
        const post_message_res = http.post(`${__ENV.BASE_API_URL}channels/${channel_id}/messages/`)
        const post_body = JSON.parse(post_message_res.body)
        const msg_id = post_body.id

        //PUT new content
        const put_params = {
            headers: {
                'Content-Type': 'application/json'
            }
        }

        const put_body = {
            content: "Automated k6 test message",
            type: "text"
        }

        const put_message_res =
            http.put(`${__ENV.BASE_API_URL}channels/${channel_id}/messages/${msg_id}`, JSON.stringify(put_body), put_params)
        if (put_message_res.status !== 200) {
            console.log(put_message_res.status)
            console.log("PUT failed... skipping!")
            console.log(put_message_res.body)
            continue;
        }

        // GET updated content
        const get_message_res = http.get(`${__ENV.BASE_API_URL}channels/${channel_id}/messages/`)
        if (get_message_res.status !== 200) {
            console.log(get_message_res.status)
            console.log("GET failed... skipping!")
            console.log(get_message_res.body)
            continue;
        }


        // DELETE message
        const del_message_res = http.del(`${__ENV.BASE_API_URL}channels/${channel_id}/messages/${msg_id}`);
        if (del_message_res.status !== 200) {
            console.log(del_message_res.status)
            console.log("GET failed... skipping!")
            console.log(del_message_res.body)
        }
        sleep(0.1)
    }

    // DELETE Request
    const response_del = http.del(`${__ENV.BASE_API_URL}channels/${channel_id}`);
    if (response_del.status !== 200) {
        console.log(response_del.status)
        console.log("GET failed... skipping!")
        console.log(response_del.body)
    }
    sleep(0.5);
}

export function handleSummary(data) {
    return {'./out/crud-messages.json': JSON.stringify(data)};
}

