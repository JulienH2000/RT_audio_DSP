import * as Band from './band.js';
import { getLogValue, getLinearGain } from './utils.js';

var trim = document.getElementById("trim");
var trimoutput= document.getElementById("trimvalue");

var statusled = document.getElementById("statusled");

var peak1 = document.getElementById("peakled1");
var peakoutput1 = document.getElementById("peakvalue1");
var peak2 = document.getElementById("peakled2");
var peakoutput2 = document.getElementById("peakvalue2");

var rms1 = document.getElementById("rmsled1");
var rmsoutput1 = document.getElementById("rmsvalue1");
var rms2 = document.getElementById("rmsled2");
var rmsoutput2 = document.getElementById("rmsvalue2");

var bypass_default_color = "#268cc7";

var bypass_lpf = document.getElementById("bypass_lpf");
var bp_l_state = false;
bypass_lpf.addEventListener("click", updatebp_lpf);
bypass_lpf.style.backgroundColor=bypass_default_color;


const baseUrl = "10.67.0.24:6060/";

trim.innerHTML = trim.value + 'dB';

trim.oninput = function() { {postInfo("trim", "amp", getLinearGain(trim.value))};
  trimoutput.innerHTML = this.value + 'dB';
}
trim.ondblclick = function() { 
  trim.value = 0;
  {postInfo("trim", "amp", getLinearGain(trim.value))};
  trimoutput.innerHTML = trim.value + 'dB';
}

bandf.oninput = function() { {postInfo("band", "freq", getLogValue(bandf.value))};
  bandfoutput.innerHTML = getLogValue(this.value) + 'Hz';
}


banda.oninput = function() { {postInfo("band", "amp", getLinearGain(banda.value))};
  bandaoutput.innerHTML = this.value + 'dB';
}
banda.ondblclick = function() { 
  banda.value = 0;
  {postInfo("band", "amp", getLinearGain(banda.value))};
  bandaoutput.innerHTML = banda.value + 'dB';
}


bandq.oninput = function() { {postInfo("band", "q", bandq.value)};
  bandqoutput.innerHTML = this.value;
}
bandq.ondblclick = function() { 
  bandq.value = 1;
  {postInfo("band", "q", bandq.value)};
  bandqoutput.innerHTML = bandq.value;
}

async function postInfo(target, param, value) {
  const res = await fetch ('/',
  {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      "content-Type": 'application/json'
    },
    body: JSON.stringify ({
      target: target,
      param: param,
      value: value
    })
  })
  //console.log("send to lpf")
}

async function getStatus() {
  const res = await fetch('/dsp-status', {
    method: 'GET',
    headers: {
      'Content-type': 'application/json'
    }
  })
  .then(res => res.json())
  .then(res => {
    return res.ping
  })
  //console.log(res)
  return res
}

var ping_dsp = window.setInterval(async function(){
  var status = getStatus();
  if (await status) {
    statusled.style.backgroundColor="#00ff00";
    statusled.style.borderBlockColor="#68f768";
  } else {
    statusled.style.backgroundColor="#ff0000";
    statusled.style.borderBlockColor="#db5858";
  }
}, 1000);

async function getStatus() {
  try {
    const response = await fetch('/status', {
      method: 'GET',
      headers: {
        'Content-type': 'application/json'
      }
    });
    const data = await response.json();
    //console.log("Raw response data:", data);
    if (!data) {
      console.warn("No Status data in response");
    }
    return data;
  } catch (error) {
    console.error("Error in getPeak:", error);
    throw error;
  }
}

const PEAK_LEVELS = {
  HIGH: -6,
  MEDIUM: -18,
  LOW: -60
};

const COLORS = {
  RED: "#ff0000",
  ORANGE: "#ff8000",
  GREEN: "#00ff00",
  GRAY: "#3f3f3f"
};

function updateLevelDisplay(peakElement, peakValue) {
  if (peakValue > PEAK_LEVELS.HIGH) {
    peakElement.style.backgroundColor = COLORS.RED;
  } else if (peakValue > PEAK_LEVELS.MEDIUM) {
    peakElement.style.backgroundColor = COLORS.ORANGE;
  } else if (peakValue > PEAK_LEVELS.LOW) {
    peakElement.style.backgroundColor = COLORS.GREEN;
  } else {
    peakElement.style.backgroundColor = COLORS.GRAY;
  }
}

var get_peak = window.setInterval(async function () {
  try {
    const status = await getStatus();
    console.log(status);
    
    updateLevelDisplay(peak1, status.peakchannel1);
    peak1.innerHTML = Math.trunc(status.peakchannel1)
    updateLevelDisplay(peak2, status.peakchannel2);
    peak2.innerHTML = Math.trunc(status.peakchannel2)
    updateLevelDisplay(rms1, status.rmschannel1);
    rms1.innerHTML = 'L ' + Math.trunc(status.rmschannel1)
    updateLevelDisplay(rms2, status.rmschannel2);
    rms2.innerHTML = 'R ' + Math.trunc(status.rmschannel2)
  } catch (error) {
    console.error("Error getting status:", error);
  }
}, 120);
