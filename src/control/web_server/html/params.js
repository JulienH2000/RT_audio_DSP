import { getLogValue, getLinearValue, revertLinear, getLinearFromLog } from './utils.js';
import { postInfo } from './api.js';

export function Band(subtarget, f, a, q) {
    this.freq = new Rotary("eq", subtarget, "freq", f, f, 'Hz');
    this.gain = new Rotary("eq", subtarget, "amp", a, a, 'dB');
    this.q = new Rotary("eq", subtarget, "q", q, q, '');
}

export function Rotary(target, subtarget, param, value, def, suffix) {
    this.target = target;
    this.subtarget = subtarget;
    this.param = param;
    this.def = def;
    this.value = value;
    this.display_value = this.value + suffix;
    this.input = document.getElementById(subtarget + param);
    this.display = document.getElementById(subtarget + param + "_display");
    this.suffix = suffix;
}

Rotary.prototype.init = function(v) {
    switch (this.suffix) {
        case 'Hz' : v = getLinearFromLog(v); 
            break; 
        default : v = v; 
            break;
    }
    this.input.value = v;
    this.update(v);
}

Rotary.prototype.update = function(v) {
    switch (this.suffix) {
        case 'Hz' : v = getLogValue(v); 
            break; 
        default : v = v; 
            break;
    }
    this.value = v;
    this.display_value = v + this.suffix;
    this.display.innerHTML = this.display_value;
    if (this.suffix === 'dB') { v = getLinearValue(v)}
    postInfo(this.target, this.subtarget, this.param, v);
  };
  
Rotary.prototype.oninput = function() {
    this.input.addEventListener('input', (event) => {
            this.update(event.target.value);
        });
    this.input.addEventListener('dblclick', (event) => {
            this.init(this.def)
        });
  };