class Diary {
    private static readonly TEXT_LIMIT = 39;

    private currentLine: string;
    private stack: string[];

    constructor() {
        this.currentLine = "";
        this.stack = [];
    }

    set text(arg: string) {
        this.currentLine = arg;

        if (this.currentLine.length > Diary.TEXT_LIMIT) {
            this.addLine(this.currentLine);
        }
    }

    get text(): string {
        return this.currentLine;
    }

    get lines(): string[] {
        return this.stack;
    }

    // To be sent to the server.
    get fullText(): string {
        let allLines = [...this.stack, this.currentLine];
        return allLines.join("\n");
    }

    private addLine(text: string) {
        // Store everything except the last word that is currently being typed.
        let splittedText = text.split(" ");
        let lastWord = splittedText.pop() as string;
        let remainder = splittedText.join(" ");

        this.stack.push(remainder);
        this.currentLine = lastWord;
    }

    // When the user presses backspace until the
    // whole line is cleared it should go back to the previous line.
    public removeLine() {
        console.log("remove");

        if (this.currentLine === "") {
            this.currentLine = this.stack.pop() as string;
            console.log("new line");
        }
    }

    public init(fullText: string) {
        let split = fullText.split("\n");

        if (split.length == 0)
            return

        this.currentLine = split.pop() as string;
        this.stack = split;
    }
}

export default Diary;
