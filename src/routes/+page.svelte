<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { onMount, onDestroy } from 'svelte';

  interface SerialMessage {
    text: string;
    timestamp: Date;
  }

  // State variables
  let ports: string[] = [];
  let selectedPort = '';
  let baudRates = [9600, 115200, 230400, 460800, 921600];
  let selectedBaud = 115200;
  let message = '';
  let isConnected = false;
  let messages: SerialMessage[] = [];
  let partialMessage: SerialMessage | null = null;
  let unsubscribe: () => void;

  // Port handling
  const refreshPorts = async () => {
    ports = await invoke('list_ports');
  };

  // Connection handling
  const connectSerial = async () => {
    try {
      await invoke('connect', { portName: selectedPort, baudRate: selectedBaud });
      isConnected = true;
      setupListener();
    } catch (err) {
      alert(`Connection error: ${err}`);
    }
  };

  // Message listener
  const setupListener = async () => {
    unsubscribe = await listen<string>('serial-data', (event) => {
      const timestamp = new Date();
      const buffer = event.payload;
      const lines = buffer.split('\n');
      
      // Process complete lines
      if (lines.length > 1) {
        const newMessages = lines.slice(0, -1).map(text => ({
          text,
          timestamp: new Date(timestamp.getTime() - (lines.length * 10))
        }));
        messages = [...newMessages, ...messages];
      }

      // Handle partial line
      partialMessage = lines[lines.length - 1] 
        ? { text: lines[lines.length - 1], timestamp }
        : null;
    });
  };

  // Message sending
  const sendMessage = async () => {
    if (message.trim()) {
      await invoke('send_message', { message: message + '\n' });
      message = '';
    }
  };

  // Clear messages
  const clearMessages = () => {
    messages = [];
    partialMessage = null;
  };

  // Lifecycle
  onMount(refreshPorts);
  onDestroy(() => unsubscribe?.());
</script>

<div class="container">
  <h1>Serial Monitor</h1>
  
  <div class="controls">
    <select bind:value={selectedPort} disabled={isConnected}>
      <option value="">Select Port</option>
      {#each ports as port}
        <option value={port}>{port}</option>
      {/each}
    </select>
    
    <select bind:value={selectedBaud} disabled={isConnected}>
      {#each baudRates as rate}
        <option value={rate}>{rate}</option>
      {/each}
    </select>
    
    <button on:click={connectSerial} disabled={!selectedPort || isConnected}>
      {isConnected ? 'Connected' : 'Connect'}
    </button>
    
    <button on:click={clearMessages} class="danger">Clear</button>
  </div>

  <div class="terminal">
    {#if partialMessage}
      <div class="message">
        <span class="timestamp">{partialMessage.timestamp.toLocaleTimeString()}</span>
        <span class="text">{partialMessage.text}</span>
      </div>
    {/if}

    {#each messages as msg}
      <div class="message">
        <span class="timestamp">{msg.timestamp.toLocaleTimeString()}</span>
        <span class="text">{msg.text}</span>
      </div>
    {/each}
  </div>

  <div class="send-container">
    <input 
      bind:value={message} 
      placeholder="Enter message..." 
      on:keydown={(e) => e.key === 'Enter' && sendMessage()}
    />
    <button on:click={sendMessage} disabled={!isConnected}>Send</button>
  </div>
</div>

<style>
  .container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    height: 100vh;
    display: flex;
    flex-direction: column;
  }

  h1 {
    text-align: center;
    margin-bottom: 1rem;
  }

  .controls {
    display: flex;
    gap: 10px;
    margin-bottom: 20px;
    flex-wrap: wrap;
  }

  select, button {
    padding: 8px 12px;
    border-radius: 4px;
    border: 1px solid #ccc;
  }

  button.danger {
    background-color: #ff4444;
    color: white;
  }

  .terminal {
    flex: 1;
    border: 1px solid #ccc;
    padding: 10px;
    overflow-y: auto;
    font-family: monospace;
    display: flex;
    flex-direction: column-reverse;
  }

  .message {
    display: flex;
    gap: 1rem;
    padding: 2px 0;
    animation: fadeIn 0.3s ease-in;
  }

  .timestamp {
    color: #666;
    min-width: 80px;
    font-size: 0.9em;
  }

  .text {
    flex: 1;
    word-break: break-all;
  }

  .send-container {
    margin-top: 20px;
    display: flex;
    gap: 10px;
  }

  input {
    flex: 1;
    padding: 8px;
    border-radius: 4px;
    border: 1px solid #ccc;
  }

  @keyframes fadeIn {
    from { opacity: 0; transform: translateY(-10px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
