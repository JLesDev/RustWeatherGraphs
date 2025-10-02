import init, { /*get_weather_name, get_url,*/ call_prog, show_line_ticks} from './pkg/hot_or_not_3.js';
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

function cityText(city){
  let parent = document.querySelector('#placeholder2');
  switch(city){
    case 0:
      parent.textContent = "Melbourne";
      break;
    case 1:
      parent.textContent = "Adelaide";
      break;
    case 2: 
      parent.textContent = "Sydney";
      break;
    case 3:
      parent.textContent = "Brisbane";
      break;
  }
 
}

const urls = [
  "https://api.weather.bom.gov.au/v1/locations/r1r143/forecasts/hourly", //MELBOURNE
  "https://api.weather.bom.gov.au/v1/locations/r1f966/forecasts/hourly", //ADELAIDE
  "https://api.weather.bom.gov.au/v1/locations/r3gwbq/forecasts/hourly", //SYDNEY
  "https://api.weather.bom.gov.au/v1/locations/r7hgdm/forecasts/hourly" //BRISBANE
];
  let miner = 999;
  let maxer = -999;

async function run(city) {
  //location.reload();
  await init();
  doSomething();
  let a = await call_prog();
  console.log(a);
  //test("https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json");

  // search for geohashes: https://api.weather.bom.gov.au/v1/locations?search=2020

  const url = "https://api.weather.bom.gov.au/v1/locations/r1r143/forecasts/hourly";

  const url2 = "https://api.weather.bom.gov.au/v1/locations/r1r143/forecasts/hourly";

  const headers = {
    'Content-Type': 'application/json',
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Methods': 'POST,PATCH,OPTIONS'
  }

  const response = {
    statusCode: 200,
    headers: headers,
    body: JSON.stringify(urls[city]),
  };

  const responses = {
    statusCode: 200,
    headers: headers,
    body: JSON.stringify(url2),
  };

  //document.body.textContent = "hi";

  console.log(response);

  const xValues = [];
  const yValues = [];
  const xValues2 = [];
  const yValues2 = [];

  

  await fetch(String(urls[2])) //1
    .then((response) => response.json()) //2
    .then((observations) => {
      console.log(observations);
      console.log(observations.data[0].temp);
      console.log(observations.notice);
      console.log("test");
      
      let ci = observations.data[0].temp; //3
      let ci2 = observations.data[0].temp_feels_like;
      let parent = document.querySelector('#placeholder2');
      const barColors = ["red", "green","blue","orange","brown"];
      
      console.log("jo test" + window.callbacks.show_line_ticks);

      for (let i = 0; i < 1; i++){
        let ci = observations.data[i].temp; 

        let p = document.createElement('p');
        let timer = new Date(); 
        let realtimer = new Date(observations.data[i].time);
        console.log(realtimer);
        let now = new Date();
        let currentTime = realtimer.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
        console.log(currentTime.getHours);
        console.log(`Current Time: ${currentTime}`);
        let nozero = realtimer.getHours().toString();
        let nonzero = nozero.replace(/^0+/, '');
        console.log(nonzero+":"+realtimer.getMinutes());
        let formatTime = nozero;
        console.log("nozerotime = "+nozero.toString());
        let date = new Date();
        let dateform = date.toLocaleDateString();
        switch(nozero.toString()){
          case "0": formatTime = dateform;
            break;
          case "1": formatTime = "1am";
            break;
          case "2": formatTime = "2am";
            break;
          case "3": formatTime = "3am";
            break;
          case "4": formatTime = "4am";
            break;
          case "5": formatTime = "5am";
            break;
          case "6": formatTime = "6am";
            break;
          case "7": formatTime = "7am";
            break;
          case "8": formatTime = "8am";
            break;
          case "9": formatTime = "9am";
            break;
          case "10": formatTime = "10am";
            break;
          case "11": formatTime = "11am";
            break;
          case "12": formatTime = "12pm";
            break;
          case "13": formatTime = "1pm";
            break;
          case "14": formatTime = "2pm";
            break;
          case "15": formatTime = "3pm";
            break;
          case "16": formatTime = "4pm";
            break;
          case "17": formatTime = "5pm";
            break;
          case "18": formatTime = "6pm";
            break;
          case "19": formatTime = "7pm";
            break;
          case "20": formatTime = "8pm";
            break;
          case "21": formatTime = "9pm";
            break;
          case "22": formatTime = "10pm";
            break;
          case "23": formatTime = "11pm";
            break;
          defualt: "Unknown time!";
            break;
        }
        console.log("formatTime = "+formatTime);
	      p.textContent = formatTime + ". Temp: " + observations.data[i].temp + "°C. Jonty's Temp: " + (observations.data[i].temp_feels_like + 3)+"°C.";

	      parent.appendChild(p);
        let q = document.createElement('p');
        q.textContent = "UV: " + observations.data[i].uv + ". Humidity: " + (observations.data[i].relative_humidity)+ "%. Is it raining? Not quite sure.";
        parent.appendChild(q);
      }

      for (let i = 24; i > -1; i--){
        let ci = observations.data[i].temp; 
        if(ci < miner){
          miner = ci;
        }
        if(ci > maxer){
          maxer = ci;
        }
        console.log(ci);
        let p = document.createElement('p');
        //let timer = new Date(observations.data[i].time);
        let timer = new Date(); 
        let realtimer = new Date(observations.data[i].time);
        console.log(realtimer);
        let now = new Date();
        let currentTime = realtimer.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
        console.log(currentTime.getHours);
        console.log(`Current Time: ${currentTime}`);
        let nozero = realtimer.getHours().toString();
        let nonzero = nozero.replace(/^0+/, '');
        console.log(nonzero+":"+realtimer.getMinutes());
	      //p.textContent = currentTime + ". Temp: " + observations.data[i].temp + "°C. Jonty's Temp: " + observations.data[i].temp_feels_like;
        
        yValues[i] = observations.data[i].temp;

        let formatdate = new Date();
        let nextdate = formatdate.toLocaleDateString();
        
        if (observations.data[i].time === "undefined"){
          xValues[i] = "Not found";
        }
        else{
          let formatTime = nozero;
          switch(nozero.toString()){
            case "0": formatTime = nextdate;
              break;
            case "1": formatTime = "1am";
              break;
            case "2": formatTime = "2am";
              break;
            case "3": formatTime = "3am";
              break;
            case "4": formatTime = "4am";
              break;
            case "5": formatTime = "5am";
              break;
            case "6": formatTime = "6am";
              break;
            case "7": formatTime = "7am";
              break;
            case "8": formatTime = "8am";
              break;
            case "9": formatTime = "9am";
              break;
            case "10": formatTime = "10am";
              break;
            case "11": formatTime = "11am";
              break;
            case "12": formatTime = "12pm";
              break;
            case "13": formatTime = "1pm";
              break;
            case "14": formatTime = "2pm";
              break;
            case "15": formatTime = "3pm";
              break;
            case "16": formatTime = "4pm";
              break;
            case "17": formatTime = "5pm";
              break;
            case "18": formatTime = "6pm";
              break;
            case "19": formatTime = "7pm";
              break;
            case "20": formatTime = "8pm";
              break;
            case "21": formatTime = "9pm";
              break;
            case "22": formatTime = "10pm";
              break;
            case "23": formatTime = "11pm";
              break;
            defualt: "Unknown time!";
              break;
          };
          xValues[i] = formatTime;
        }
        
	      parent.appendChild(p);
      }

     
      }

      
    )
    .then((temp) => {

    }
    );



  await fetch(String(urls[2])) //1
    .then((response) => response.json()) //2
    .then((observations) => {
      console.log(observations);
      console.log(observations.data[0].temp);
      console.log(observations.notice);
      console.log("test");
      
      let ci = observations.data[0].temp; //3
      let ci2 = observations.data[0].temp_feels_like;
      let parent = document.querySelector('#placeholder2');
      const barColors = ["red", "green","blue","orange","brown"];
      
      console.log("jo test" + window.callbacks.show_line_ticks);


      for (let i = 0; i < 1; i++){

        

        let ci = observations.data[i].temp; 

        let p = document.createElement('p');
        let timer = new Date(); 
        let realtimer = new Date(observations.data[i].time);
        console.log(realtimer);
        let now = new Date();
        let currentTime = realtimer.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
        console.log(currentTime.getHours);
        console.log(`Current Time: ${currentTime}`);
        let nozero = realtimer.getHours().toString();
        let nonzero = nozero.replace(/^0+/, '');
        console.log(nonzero+":"+realtimer.getMinutes());
        let formatTime = nozero;
        console.log("nozerotime = "+nozero.toString());
        let date = new Date();
        //let nextdate = 
        let dateform = date.toLocaleDateString();
        switch(nozero.toString()){
          case "0": formatTime = dateform;
            break;
          case "1": formatTime = "1am";
            break;
          case "2": formatTime = "2am";
            break;
          case "3": formatTime = "3am";
            break;
          case "4": formatTime = "4am";
            break;
          case "5": formatTime = "5am";
            break;
          case "6": formatTime = "6am";
            break;
          case "7": formatTime = "7am";
            break;
          case "8": formatTime = "8am";
            break;
          case "9": formatTime = "9am";
            break;
          case "10": formatTime = "10am";
            break;
          case "11": formatTime = "11am";
            break;
          case "12": formatTime = "12pm";
            break;
          case "13": formatTime = "1pm";
            break;
          case "14": formatTime = "2pm";
            break;
          case "15": formatTime = "3pm";
            break;
          case "16": formatTime = "4pm";
            break;
          case "17": formatTime = "5pm";
            break;
          case "18": formatTime = "6pm";
            break;
          case "19": formatTime = "7pm";
            break;
          case "20": formatTime = "8pm";
            break;
          case "21": formatTime = "9pm";
            break;
          case "22": formatTime = "10pm";
            break;
          case "23": formatTime = "11pm";
            break;
          defualt: "Unknown time!";
            break;
        }
        console.log("formatTime = "+formatTime);
	      //p.textContent = formatTime + ". Temp: " + observations.data[i].temp + "°C. Jonty's Temp: " + observations.data[i].temp_feels_like+"°C.";
	      parent.appendChild(p);
      }

      for (let i = 24; i > -1; i--){
        let ci = observations.data[i].temp; 

        console.log(ci);
        let p = document.createElement('p');
        let realtimer = new Date(observations.data[i].time);
        console.log(realtimer);
        let now = new Date();
        let currentTime = realtimer.toLocaleTimeString('en-US', { hour: '2-digit', minute: '2-digit', second: '2-digit' });
        console.log(currentTime.getHours);
        console.log(`Current Time: ${currentTime}`);
        let nozero = realtimer.getHours().toString();
        let nonzero = nozero.replace(/^0+/, '');
        console.log(nonzero+":"+realtimer.getMinutes());
	      //p.textContent = currentTime + ". Temp: " + observations.data[i].temp + "°C. Jonty's Temp: " + observations.data[i].temp_feels_like;
        
        yValues2[i] = observations.data[i].temp;
        let formatdate = new Date();
        //let nextdate = new Date();
        //nextdate = setDate(formatdate.getDate() + 1);
        let nextdate = new Date(new Date().getTime() + 24 * 60 * 60 * 1000);
        console.log(nextdate);
        var options = { day: 'numeric', month: 'long' };
        let noyear = (nextdate.toLocaleDateString('en-US', options)).toString();
        console.log(formatdate.toLocaleDateString('en-US', options));
        
        if (observations.data[i].time === "undefined"){
          xValues[i] = "Not found";
        }
        else{
          let formatTime = nozero;
          switch(nozero.toString()){
            case "0": formatTime = noyear;
              break;
            case "1": formatTime = "1am";
              break;
            case "2": formatTime = "2am";
              break;
            case "3": formatTime = "3am";
              break;
            case "4": formatTime = "4am";
              break;
            case "5": formatTime = "5am";
              break;
            case "6": formatTime = "6am";
              break;
            case "7": formatTime = "7am";
              break;
            case "8": formatTime = "8am";
              break;
            case "9": formatTime = "9am";
              break;
            case "10": formatTime = "10am";
              break;
            case "11": formatTime = "11am";
              break;
            case "12": formatTime = "12pm";
              break;
            case "13": formatTime = "1pm";
              break;
            case "14": formatTime = "2pm";
              break;
            case "15": formatTime = "3pm";
              break;
            case "16": formatTime = "4pm";
              break;
            case "17": formatTime = "5pm";
              break;
            case "18": formatTime = "6pm";
              break;
            case "19": formatTime = "7pm";
              break;
            case "20": formatTime = "8pm";
              break;
            case "21": formatTime = "9pm";
              break;
            case "22": formatTime = "10pm";
              break;
            case "23": formatTime = "11pm";
              break;
            defualt: "Unknown time!";
              break;
          };
          xValues[i] = formatTime;
        }
        
	      parent.appendChild(p);
      }

     
      }

      
    )
    .then((temp) => {

    }
    );

   function addData(chart, label, newData) {
        chart.data.labels.push(label);
        chart.data.datasets.forEach((dataset) => {
            dataset.data.push(newData);
        });
        chart.update();
      }

      // function removeData(chart) {
      //   chart.data.labels.pop();
      //   chart.data.datasets.forEach((dataset) => {
      //       dataset.data.pop();
      //   });
      //   chart.update();
      // }
 
      const plugin = {
        id: 'customCanvasBackgroundColor',
        beforeDraw: (chart, args, options) => {
          const {ctx} = chart;
          ctx.save();
          ctx.globalCompositeOperation = 'destination-over';
          ctx.fillStyle = options.color || '#99ffff';
          ctx.fillRect(0, 0, chart.width, chart.height);
          ctx.restore();
        }
      };

      new Chart("myChart", {
        type: "line",
        //title: "hi",
        data: {
          labels: xValues,
          datasets: [{
            label: '',
            pointRadius: 0,
            //color: "white",
            borderColor: 'rgba(255, 230, 149, 1)',       // Line color
            //backgroundColor: "black",   // Point color
            //pointBackgroundColor: "black",
            //pointBorderColor: "black",
            fill: true,
            backgroundColor: 'rgba(85, 85, 85, 1)',
            data: yValues
          }
          // {
          //   label: "",
          //   //color: "white",
          //   borderColor: "lightGrey",       // Line color
          //   //backgroundColor: "black",   // Point color
          //   //pointBackgroundColor: "black",
          //   //pointBorderColor: "black",
          //   fill: true,
          //   backgroundColor: 'rgba(85, 85, 85, 1)',
          //   data: yValues2
          // }
        
          ]
        },
        options: {
          label: {
            display: false
          },
          legend: {
            display: false
          },
          tooltips: {
            enabled: false
          },
          
          //backgroundColor: 'black',
          plugins: {
            // maintainAspectRatio: false,
            customCanvasBackgroundColor: {
                color: '#121213',
              },
            legend: {
              display: false
            },
          },
          datalabels: {
            display: false
          },
          title: {
            display: false,
            fontColor: 'white',
            text: "yessir",
            //text: observations.data[0].name + " temperature for " + observations.data[0].time,
            //text: "WEATHER"
          },
          scales: {
            x: {
              
              title: false,
              ticks: {
                color: 'rgba(190, 190, 190, 1)'
              }
            },
            y: {
              max: maxer + 2,
              min: miner - 2,
              ticks: {
                //labels: yValues + "cdcd",
                //display: false,
                color: 'rgba(190, 190, 190, 1)'
              }
            },
          },
        },
         
        plugins: [plugin],
        });
        //Chart.defaults.global.defautFontColor = 'white';

        //removeData(myChart);
        // myChart.destroy();
        // addData(myChart, yValues, xValues);


  var z = document.createElement("h2");
  z.setAttribute("id", "placeholder");
  document.body.appendChild(z);

  // var feels = document.createElement("h2");
  // feels.setAttribute("id", "feels");
  // document.getElementById("feels-div").appendChild(feels);
  // document.getElementById("feels").textContent = "Feels like";


    

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


document.getElementById("melb").addEventListener("click", function (e) {
  runner(0);
  window.location.href = "index.html";
  // removeData("myChart");
});

document.getElementById("adel").addEventListener("click", function (e) {
  runner(1);
  window.location.href = "adelaide.html";
});

document.getElementById("sydn").addEventListener("click", function (e) {
  runner(2);
  window.location.href = "sydney.html";
});

document.getElementById("bris").addEventListener("click", function (e) {
  runner(3);
  window.location.href = "brisbane.html";
});

document.getElementById("more").addEventListener("click", function (e) {
  runner(3);
  window.location.href = "more.html";
});

function runner(city){
  cityText(city);
  console.log("RUNNER");
  location.reload;
  run(city);
}

run(1);
window.run = run;
// location.reload();