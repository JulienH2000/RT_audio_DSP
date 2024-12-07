export function getLogValue(value) {
    const minLog = Math.log10(20);
    const maxLog = Math.log10(20000);
    const scale = (maxLog - minLog) / (20000 - 20);
    
    return Math.trunc(Math.pow(10, minLog + (value - 20) * scale));
}
  
export function getLinearValue(logGain) {
    return Math.pow(10,logGain / 20)
}

export function revertLinear(value) {
  return 20 * Math.log10(value);
}

export function getLinearFromLog(logValue) {
  const minLog = Math.log10(20);
  const maxLog = Math.log10(20000);
  const scale = (maxLog - minLog) / (20000 - 20);

  // Reverse the logarithmic scaling
  return 20 + (Math.log10(logValue) - minLog) / scale;
}