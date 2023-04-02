<script lang="ts">
import { onMount } from 'svelte';
import Diary from './diary';
import { PUBLIC_BASE_URL, PUBLIC_SECRET } from '$env/static/public';

let connected = false;
let loaded = false;

let diary = new Diary();

const opacity: number[] = [
    .05,
    .5,
    .98,
];

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
    // Surpress Enter (TODO: Should create a new line)
    if (e.key == 'Enter')
        e.preventDefault()

    if (e.key == 'Backspace') {
        diary.removeLine()
        diary.text = diary.text; // Trigger redraw
    }

    // Send each change to the server.
    if (connected && loaded)
        socket.send(diary.fullText);
}
</script>

<div>
    {#each diary.lines as line, i}
        <p class="text" style="opacity:{opacity[i]}">{line}</p>
    {/each}

    <textarea class="text" bind:value={diary.text} on:keydown={keydown} autofocus />
</div>

<style>
@font-face {
    font-family: 'Alegreya';
    font-style: normal;
    font-weight: 100;
    src: local('Alegreya'), url('/fonts/Alegreya.ttf') format('truetype');
}


div {
    display: flex;
    flex-flow: column;

    justify-content: center;
    align-items: center;
    height: 100vh;
}

.text {
    width: 40%;
    height: 59px;

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
}
</style>
