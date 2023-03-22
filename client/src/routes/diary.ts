class Diary {
    public static readonly TEXT_LIMIT = 39;

    private currentLine: string;

    constructor() {

    }

    set text(arg: string) {
        this.currentLine = arg;

        if (this.currentLine.length > Diary.TEXT_LIMIT) {
            this.moveLineUp()
        }
    }

    private moveLineUp() {
        // The last word should be not be split up.
        let remainder = text.split(" ");
        let lastWord = remainder.pop() as string;

        lines.push(remainder.join(" "));
        lines.shift();

        text = lastWord;
    }
}

export default Diary;
