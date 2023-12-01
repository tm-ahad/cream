class IdGen {
    static gen() {
        return IdGen.id++;
    }
}
IdGen.id = 0;
