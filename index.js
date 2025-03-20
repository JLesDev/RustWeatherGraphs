import init, { /*get_weather_name, get_url,*/ test, call_prog, show_line_ticks, main_js} from './pkg/hot_or_not_3.js';
await init();

// src="https://cdnjs.cloudflare.com/ajax/libs/Chart.js/2.9.4/Chart.js";

import * as root from './pkg/hot_or_not_3.js'

//const cors = require('cors');

//app.use(cors());

// myChart.destroy();
window.callbacks = root;
console.log("test");
console.log(window.callbacks);
window.mutate_chart_object = function (v) {
  if (v.id == ("bar")) {
    v.options.scales.y1.ticks = {
      callback:
        function (value, _index, _values) {
          return '$' + value.toFixed(2);
        }
    };
  }
  //v.destroy();
  return v
};

async function run() {
  await init();
  doSomething();
  let a = await call_prog();
  console.log(a);
  test("https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json");
  const url = "https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json";
  const urls = [
    "https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json",
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94866.json",
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94854.json"

  ];
  const url2 = "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94866.json";
  const headers = {
    'Content-Type': 'application/json',
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'POST,PATCH,OPTIONS'
  }

  const response = {
    statusCode: 200,
    headers: headers,
    body: JSON.stringify(url),
  };

  const responses = {
    statusCode: 200,
    headers: headers,
    body: JSON.stringify(url2),
  };

  //document.body.textContent = "hi";

  console.log(response);


  // await fetch(String(url)) //1
  //   .then((response) => response.json()) //2
  //   .then((observations) => {
  //     console.log(observations);
  //     console.log(observations.observations.data[0].air_temp);
  //     console.log(observations.notice);
  //     console.log("test");
      
  //     let ci = observations.observations.data[0].air_temp; //3
  //     let ci2 = observations.observations.data[0].apparent_t;
  //     let parent = document.querySelector('#placeholder');
  //     const xValues = [];
  //     const yValues = [];
  //     const barColors = ["red", "green","blue","orange","brown"];
      
  //     console.log("jo test" + window.callbacks.show_line_ticks);
  //     for (let i = 47; i > 0; i--){
  //       let ci = observations.observations.data[i].air_temp; 
  //       console.log(ci);
  //       let p = document.createElement('p');
	//       p.textContent = observations.observations.data[i].local_date_time + " " + observations.observations.data[i].air_temp + " " + observations.observations.data[i].name;
  //       yValues[i] = observations.observations.data[i].air_temp;
        
  //       if (observations.observations.data[i].local_date_time === "undefined"){
  //         xValues[i] = "Not found";
  //       }
  //       else{
  //         xValues[i] = observations.observations.data[i].local_date_time;
  //       }
        
	//       parent.appendChild(p);
  //     }

  //     new Chart("myChart", {
  //       type: "line",
  //       data: {
  //         labels: xValues,
  //         datasets: [{
  //           fill: false,
  //           backgroundColor: "black",
  //           data: yValues
  //         }]
  //       },
  //       options: {
  //       legend: {display: false},
  //       title: {
  //         display: true,
  //         text: observations.observations.data[0].name + " temperature for " + observations.observations.data[0].local_date_time,
  //       }}
  //     });
  //     })
  //   .then((temp) => {

  //   }
  //   );


  var z = document.createElement("h1");
  z.setAttribute("id", "placeholder");
  document.body.appendChild(z);

  var feels = document.createElement("h2");
  feels.setAttribute("id", "feels");
  document.getElementById("feels-div").appendChild(feels);
  document.getElementById("feels").textContent = "Feels like";


    

  async function doSomething() {
    // let result = await get_weather_name("London");
    // console.log(result);
    // return result;
  }

  // const address = get_weather_name("London")
  //   .then((response) => get_weather_name("London"))
  //   .then((user) => {
  //     return user.name;
  //   });

  // const printAddress = async () => {
  //   const a = await address;
  //   console.log(a);
  // };

  function readMore() {
    var dots = document.getElementById("dots");
    var moreText = document.getElementById("more");
    var btnText = document.getElementById("myBtn");
  
    if (dots.style.display === "none") {
      dots.style.display = "inline";
      btnText.innerHTML = "Display more"; 
      moreText.style.display = "none";
    } else {
      dots.style.display = "none";
      btnText.innerHTML = "Display less"; 
      moreText.style.display = "inline";
    }
  }

  // printAddress();

}
run();
