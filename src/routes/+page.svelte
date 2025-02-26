<script lang="ts">
  import { onMount } from "svelte";
  import { getSerialPorts, sendSerialData, readSerialData, type SerialPort, type BaudRate } from "$lib/serial";

  let ports: SerialPort[] = [];
  let selectedPort: SerialPort = "";
  let baudRate: BaudRate = "9600";
  let message = "";
  let log: string[] = [];
  let reading = false;
  let intervalId: NodeJS.Timeout | null = null;

  // Fetch available serial ports on mount
  onMount(async () => {
    ports = await getSerialPorts();
  });

  async function sendMessage() {
    if (!selectedPort) return alert("Select a port first!");
    await sendSerialData(selectedPort, baudRate, message);
  }

  async function readSerial() {
    if (!selectedPort) return alert("Select a port first!");
    const data = await readSerialData(selectedPort, baudRate);
    log = [...log, data];
  }

  function startReading() {
    if (!selectedPort) return alert("Select a port first!");
    reading = true;
    intervalId = setInterval(async () => {
      const data = await readSerialData(selectedPort, baudRate);
      if (data) {
        log = [...log, data];
      }
    }, 500); // Adjust read interval as needed
  }

  function stopReading() {
    reading = false;
    if (intervalId) {
      clearInterval(intervalId);
      intervalId = null;
    }
  }
</script>
<svelte:head>Serialmon</svelte:head>
<h1>Serial Monitor</h1>

<!-- Serial Port Selection -->
<label>Select Port:</label>
<select bind:value={selectedPort}>
  <option disabled selected>Select a port</option>
  {#each ports as port}
    <option value={port}>{port}</option>
  {/each}
</select>

<!-- Baud Rate Selection -->
<label>Baud Rate:</label>
<select bind:value={baudRate}>
  <option value="9600">9600</option>
  <option value="115200">115200</option>
  <option value="57600">57600</option>
  <option value="38400">38400</option>
  <option value="19200">19200</option>
  <option value="4800">4800</option>
</select>

<!-- Input Field for Messages -->
<label>Send Data:</label>
<input type="text" bind:value={message} />
<button on:click={sendMessage}>Send</button>

<!-- Read Data Buttons -->
<button on:click={readSerial}>Read Once</button>
<button on:click={startReading} disabled={reading}>Start Reading</button>
<button on:click={stopReading} disabled={!reading}>Stop Reading</button>

<!-- Display Received Data -->
<ul>
  {#each log as line}
    <li>{line}</li>
  {/each}
</ul>

