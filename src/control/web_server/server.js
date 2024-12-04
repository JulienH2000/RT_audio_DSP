const http = require("http");
const express = require('express');
const path = require('path');

var dsp_status = false;
var peak = 0;

const host = "tcp://127.0.0.1:";
const port = 6060;

const app = express();

//app.get('/', (req, res) => {
//    res.sendFile(path.join(__dirname, 'html/index.html'));
//});
app.use(express.static(path.join(__dirname, 'html')));
app.use(express.json());



const PORT = process.env.PORT || port;

app.listen(PORT, () => {
    console.log('Server is running on port 6060');
});

app.post('/', (req,res) => {
    const parcel = req.body;

    run_socket(parcel);

    if (!parcel) {
        return res.status(400).send({status: 'failed'})
    }
    res.status(200).send({status : 'received'})
});

app.get('/dsp-status', (req, res) => {
    ping_dsp();
    res.send(JSON.stringify({ping: dsp_status}));
});

app.get('/status', async (req, res) => {
    const query = req.query;
    const peak = await getState();
    res.send(peak)
});

const net = require('net');
const fs = require('fs').promises;
const zmq = require('zeromq');
const { stringify } = require("querystring");
const { time } = require("console");

async function run_socket(parcel) {
    const sock = new zmq.Request();
    var port = 6061;
    sock.connect(host + port);
    console.log("\nProducer bound to port ", port);
    console.log("target:", parcel.target);

    await sock.send(parcel.target + ':' + parcel.param + ':' + parcel.value);
    console.log("sent:", parcel);

    const [result] = await sock.receive();
    return [result];
    console.log(result.toString);

}

async function ping_dsp() {
    const sock = new zmq.Request();
    sock.connect("tcp://127.0.0.1:6062");
    sock.receiveTimeout = 2000;
    //console.log("ping...");
    await sock.send('ping');

    try {
        const [res] = await sock.receive();
        dsp_status = true
    } catch {
        dsp_status = false
    }
}

async function getState() {
    const subscriber = new zmq.Subscriber();
    subscriber.connect("tcp://127.0.0.1:6063");
    subscriber.subscribe('');
    let output;
    try {
        for await (const [msg] of subscriber) {
            let jsonobj = JSON.parse(msg.toString());
            await subscriber.close();
            output = jsonobj
        }
    } catch (error) {
        console.error("Error in getState:", error);
        throw error;
    } finally {
        await subscriber.close();
    }
    //console.log(output)
    return output
}

