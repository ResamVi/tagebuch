<script lang="ts">
import { onMount } from 'svelte';
import Diary from './diary';

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
    socket = new WebSocket("ws://localhost:4123"); // TODO: Environment Variables
    socket.addEventListener("open", () => {
        connected = true;
    });

    // Set the text when we have written something already.
    socket.addEventListener("message", (event) => {
        console.log(event.data);
        diary.text = event.data;
        loaded = true;
    });
    socket.addEventListener("closed", () => {
        console.log("Closed");
    });
})

// When a user presses backspace until no text exists return to the previous line.
function keydown(e: KeyboardEvent) {
    // Send each change to the server.
    if (connected && loaded)
        socket.send(diary.fullText);

    if (e.key == 'Backspace')
        diary.removeLine()
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
    font-family: 'Vollkorn';
    font-style: normal;
    font-weight: 100;
    src: local('Vollkorn'), url('/fonts/Vollkorn.ttf') format('truetype');
}

/* Sahne. Sieht bisschen aus wie Handschrift */
@font-face {
    font-family: 'Alegreya';
    font-style: normal;
    font-weight: 100;
    src: local('Alegreya'), url('/fonts/Alegreya.ttf') format('truetype');
}

@font-face {
    font-family: 'Red Hat Mono Light';
    font-style: normal;
    font-weight: 100;
    src: local('Red Hat Mono Light'), url('/fonts/Red Hat Mono Light.ttf') format('truetype');
}

@font-face {
    font-family: 'Cormorant Light';
    font-style: normal;
    font-weight: 100;
    src: local('Cormorant Light'), url('/fonts/Cormorant Light.ttf') format('truetype');
}

/* Zu breit */
@font-face {
    font-family: 'Martel UltraLight';
    font-style: normal;
    font-weight: 100;
    src: local('Martel UltraLight'), url('/fonts/Martel UltraLight.ttf') format('truetype');
}


/* Ehh. Bisschen breit */
@font-face {
    font-family: 'Quattrocento';
    font-style: normal;
    font-weight: 100;
    src: local('Quattrocento'), url('/fonts/Quattrocento.ttf') format('truetype');
}

@font-face {
    font-family: 'Cardo';
    font-style: normal;
    font-weight: 100;
    src: local('Cardo'), url('/fonts/Cardo.ttf') format('truetype');
}

@font-face {
    font-family: 'Inter Thin';
    font-style: normal;
    font-weight: 100;
    src: local('Inter Thin'), url('/fonts/Inter.ttf') format('truetype');
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
