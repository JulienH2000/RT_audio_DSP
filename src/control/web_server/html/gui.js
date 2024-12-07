import { Band, Rotary } from './params.js';
import { getLogValue, getLinearValue } from './utils.js';
import { getStatus, dspIsAlive, postInfo } from './api.js';

var trim = document.getElementById("trim");
var trimoutput= document.getElementById("trimvalue");

trimoutput.innerHTML = trim.value + 'dB';

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

trim.oninput = function() { {postInfo("trim", "trim", "amp", getLinearValue(trim.value))};
  trimoutput.innerHTML = this.value + 'dB';
}
trim.ondblclick = function() { 
  trim.value = 0;
  {postInfo("trim", "trim", "amp", getLinearValue(trim.value))};
  trimoutput.innerHTML = trim.value + 'dB';
}

let band1 = new Band ("band1", 120, 0, 1);
band1.freq.init(120);
band1.freq.oninput();
band1.gain.init(0);
band1.gain.oninput();
band1.q.init(1);
band1.q.oninput();
let band2 = new Band ("band2", 240, 0, 1);
band2.freq.init(240);
band2.freq.oninput();
band2.gain.init(0);
band2.gain.oninput();
band2.q.init(1);
band2.q.oninput();
let band3 = new Band ("band3", 600, 0, 1);
band3.freq.init(600);
band3.freq.oninput();
band3.gain.init(0);
band3.gain.oninput();
band3.q.init(1);
band3.q.oninput();
let band4 = new Band ("band4", 1200, 0, 1);
band4.freq.init(1200);
band4.freq.oninput();
band4.gain.init(0);
band4.gain.oninput();
band4.q.init(1);
band4.q.oninput();
let band5 = new Band ("band5", 3200, 0, 1);
band5.freq.init(3200);
band5.freq.oninput();
band5.gain.init(0);
band5.gain.oninput();
band5.q.init(1);
band5.q.oninput();
let band6 = new Band ("band6", 8000, 0, 1);
band6.freq.init(8000);
band6.freq.oninput();
band6.gain.init(0);
band6.gain.oninput();
band6.q.init(1);
band6.q.oninput();

/*
var ping_dsp = window.setInterval(async function(){
  var status = dspIsAlive();
  if (await status) {
    statusled.style.backgroundColor="#00ff00";
    statusled.style.borderBlockColor="#68f768";
  } else {
    statusled.style.backgroundColor="#ff0000";
    statusled.style.borderBlockColor="#db5858";
  }
}, 1000);
*/

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
/*
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
*/
