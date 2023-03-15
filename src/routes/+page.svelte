<script lang="ts">
let text: string = '';
let lines: string[] = ["", "", ""];

const opacity: number[] = [
    .05,
    .5,
    .98,
];

$: {
    // Move line up after certain length.
    if (text.length > 39) {
        // The last word should be not be split up.
        let remainder = text.split(" ");
        let lastWord = remainder.pop() as string;

        lines.push(remainder.join(" "));
        lines.shift();

        text = lastWord;
    }

    // Trigger a redraw so that lines that were shift'd actually leave the screen.
    lines = lines;
}

// When a user presses backspace until no text exists return to the previous line.
function jumpBackLine(e: KeyboardEvent) {
    if (e.key == 'Backspace' && text == "") {
        let lastLine = lines.pop() as string;
        text = lastLine;
    }
    if (lines.length < 3) {
        lines.unshift("")
    }

    // Trigger a redraw so that lines that were shift'd actually leave the screen.
    lines = lines
}
</script>

<div>
    {#each lines as line, i}
        <p class="text" style="opacity:{opacity[i]}">{line}</p>
    {/each}

    <textarea class="text" bind:value={text} on:keydown={jumpBackLine} autofocus />
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
