<script lang="ts">
import { onMount } from 'svelte';
import Diary from './diary';
import { PUBLIC_BASE_URL, PUBLIC_SECRET } from '$env/static/public';

let connected = false;
let loaded = false;

let diary = new Diary();


let socket: WebSocket;

onMount(() => {
    let url = PUBLIC_BASE_URL === undefined ?  "ws://localhost:4123" : PUBLIC_BASE_URL;
    let secret = PUBLIC_SECRET === undefined ?  "sesam öffne dich" : PUBLIC_SECRET;

    socket = new WebSocket(url);
    socket.addEventListener("open", () => {
        socket.send(secret);
        connected = true;
    });

    // Set the text when we have written something already.
    socket.addEventListener("message", (event) => {
        diary.init(event.data);
        loaded = true

        diary.text = diary.text; // Trigger redraw
    });
    socket.addEventListener("closed", () => {
        console.log("Closed");
    });
})

// When a user presses backspace until no text exists return to the previous line.
function keydown(e: KeyboardEvent) {
    // Surpress Enter.
    if (e.key == 'Enter') {
        e.preventDefault();
        diary.newLine()
    }

    if (e.key == 'Backspace') {
        diary.removeLine()
    }

    // Because the key is pressed down and not released yet
    // the latest pressed key is not yet appended to 'diary.text'
    // and doesn't show in 'diary.fullText'.
    // => Adjust for that via '+ e.key' (but only if its not 'Backspace', 'Enter', etc.)
    let key = "";
    if ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".includes(e.key))
        key = e.key

    // Send each change to the server.
    if (connected && loaded)
        socket.send(diary.fullText + key); 

    diary.text = diary.text; // Trigger redraw
}
</script>

<div id="container">
    <div id="half">
        <div id="bottom">
            {#each diary.lines as line, i}
                <p class="text" style="opacity:{diary.opacity(i)}">{line}</p>
            {/each}

            <textarea class="text" bind:value={diary.text} on:keydown={keydown} autofocus />
        </div>
    </div>
</div>

<style>
@font-face {
    font-family: 'Alegreya';
    font-style: normal;
    font-weight: 100;
    src: local('Alegreya'), url('/fonts/Alegreya.ttf') format('truetype');
}

#container {
    position: absolute;
    width: 100%;
    height: 100%;
}

#half {
    position:relative;
    height:50%;
    display: flex; 
    justify-content: center;
}

#bottom {
    position: absolute;
    bottom: 0;
    width: 40%;
}

.text {
    width: 100%;
    font-size: 32px;
    font-family: 'Alegreya';
}

p {
    margin: 0;
}

textarea {
    border: none;
    outline: none;
    resize: none;
    width: 100%
}
</style>
