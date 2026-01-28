fn test_and() -> int {
    int a = 5;
    int b = 3;
    int c = 2;
    
    // Test with single & (bitwise AND)
    if ((a > b) & (b > c)) {
        return 1;
    }
    if (a>=b){
        return 1;
    }
    
    return 0;
}