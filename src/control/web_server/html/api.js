export async function postInfo(target, subtarget, param, value) {
    const res = await fetch ('/api/dsp/1/' + target + '/' + subtarget + '/' + param + '/',
    {
      method: 'POST',
      headers: {
        'Accept': 'application/json',
        "content-Type": 'application/json'
      },
      body: JSON.stringify ({
        target: target,
        value: value,
      })
    })
    //console.log("send to lpf")
  }
  
  export async function dspIsAlive() {
    const res = await fetch('/api/dsp-status', {
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

export async function getStatus() {
    try {
      const response = await fetch('/api/status', {
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