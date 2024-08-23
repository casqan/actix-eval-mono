import http from 'k6/http';
import { sleep } from 'k6';

export const options = {
  // A number specifying the number of VUs to run concurrently.
  vus: 1,
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

// The function that defines VU logic.
//
// See https://grafana.com/docs/k6/latest/examples/get-started-with-k6/ to learn more
// about authoring k6 scripts.
//
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

  const label = `./out/crud-channels.json`;
  let obj = {};
  obj[label] = JSON.stringify(data);
  return {'./out/crud-channels.json' : JSON.stringify(data) };
}

