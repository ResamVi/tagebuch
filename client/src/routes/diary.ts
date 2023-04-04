class Diary {
    private static readonly TEXT_LIMIT = 39;
    private static readonly LINES_SHOWN = 3;
    private static readonly MIN_OPACITY = 0.05;

    private currentLine: string;
    private stack: string[];

    private opacities: number[];

    constructor() {
        this.currentLine = "";
        this.stack = [];
        this.opacities = [];

        let increments = (1 - Diary.MIN_OPACITY) / Diary.LINES_SHOWN;
        for(let i = Diary.MIN_OPACITY; i < 1; i += increments)
            this.opacities.push(Math.trunc(i*100)/100);
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
        return this.stack.slice(-Diary.LINES_SHOWN);
    }

    // To be sent to the server.
    get fullText(): string {
        let allLines = [...this.stack, this.currentLine];
        return allLines.join("\n");
    }

    public opacity(index: number): number {
        // Handle edge cases when we have not yet all display lines filled with text.
        // This avoids the bug where the first submitted line would have lowest opacity.
        if(this.stack.length == 1)
            index += 2

        if(this.stack.length == 2)
            index += 1


        return this.opacities[index];
    }

    public newLine() {
        this.stack.push(this.currentLine);
        this.currentLine = "";
    }

    // When the user presses backspace until the
    // whole line is cleared it should go back to the previous line.
    public removeLine() {
        if (this.currentLine === "") {
            this.currentLine = this.stack.pop() as string;
        }
    }

    public init(fullText: string) {
        let split = fullText.split("\n");

        if (split.length == 0)
            return

        this.currentLine = split.pop() as string;
        this.stack = split;
    }

    private addLine(text: string) {
        // Store everything except the last word that is currently being typed.
        let splittedText = text.split(" ");
        let lastWord = splittedText.pop() as string;
        let remainder = splittedText.join(" ");

        this.stack.push(remainder);
        this.currentLine = lastWord;
    }
}

export default Diary;
