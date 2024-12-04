export function getLogValue(lpfValue) {
    // Calculate a logarithmic scaling of the lpf's output
    const minLog = Math.log10(20);
    const maxLog = Math.log10(20000);
    const scale = (maxLog - minLog) / (20000 - 20);
    
    // Convert lpfValue to a logarithmic scale
    return Math.trunc(Math.pow(10, minLog + (lpfValue - 20) * scale));
  }
  
  export function getLinearGain(logGain) {
    return Math.pow(10,logGain / 20)
  }