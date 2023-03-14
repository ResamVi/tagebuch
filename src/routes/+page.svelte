<script>
let text = '';

$: {
    if (text.length > 51) {
        console.log('full');
    }
}

</script>

<div>
    <textarea bind:value={text} autofocus />
</div>

<style>
@font-face {
    font-family: 'Inter Thin';
    font-style: normal;
    font-weight: 100;
    src: local('Inter Thin'), url('/fonts/Inter.ttf') format('truetype');
}

div {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
}

textarea {
    width: 50%;
    height: 59px;
    font-size: 32px;

    font-family: 'Inter Thin';

    border: none;
    outline: none;
}
</style>
