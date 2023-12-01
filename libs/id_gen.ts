class IdGen {
    public static id = 0;

    public static gen(): number {
        return IdGen.id++
    }
}
