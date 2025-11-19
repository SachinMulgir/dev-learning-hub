package java.dsa.basics.patterns;

public class patterns {
    public static void main(String[] args) {
        // Patterns:
        
        // 1.
        // *****
        // *****
        // *****
        // *****
        // *****
        System.out.println("\nPattern 1:");
        pattern_1();

        // 2.
        // *
        // **
        // ***
        // ****
        // *****
        System.out.println("\nPattern 2:");
        pattern_2();


        // 3.
        // 1
        // 12
        // 123
        // 1234
        // 12345
        System.out.println("\nPattern 3:");
        pattern_3();


        // 4.
        // 1
        // 22
        // 333
        // 4444
        // 55555
        System.out.println("\nPattern 4:");
        pattern_4();

        
        // 5.
        // *****
        // ****
        // ***
        // **
        // *
        System.out.println("\nPattern 5:");
        pattern_5();


        // 6.
        // 12345
        // 1234
        // 123
        // 12
        // 1
        System.out.println("\nPattern 6:");
        pattern_6();


        // 7.
        //     *
        //    ***
        //   *****
        //  *******
        // *********
        System.out.println("\nPattern 7:");
        pattern_7();


        // 8.
        // *********
        //  *******
        //   *****
        //    ***
        //     *
        System.out.println("\nPattern 8:");
        pattern_8();


        // 9.
        //     *
        //    ***
        //   *****
        //  *******
        // *********
        // *********
        //  *******
        //   *****
        //    ***
        //     *
        System.out.println("\nPattern 9:");
        pattern_9();


        // 10.
        // *
        // **
        // ***
        // ****
        // *****
        // ****
        // ***
        // **
        // *
        System.out.println("\nPattern 10:");
        pattern_10();


        // 11.
        // 1
        // 0 1
        // 1 0 1
        // 0 1 0 1
        // 1 0 1 0 1
        System.out.println("\nPattern 11:");
        pattern_11();


        // 12.
        // 1      1
        // 12    21
        // 123  321
        // 12344321
        System.out.println("\nPattern 12:");
        pattern_12();



        // 13.
        // 1
        // 2 3
        // 4 5 6
        // 7 8 9 10
        // 11 12 13 14 15
        System.out.println("\nPattern 13:");
        pattern_13();

        
        // 14.
        // A
        // AB
        // ABC
        // ABCD
        // ABCDE

        // hint : How to increment a char in java:
        char ch = 'A';
        char a = (char) (ch + 1);
        System.out.println("\n\nchar ch = 'A';\nSo, ch = '" + ch + "' \nand incrementing it:\nchar a = (char) (ch + 1);\na : " + a);

        System.out.println("\nPattern 14:");
        pattern_14();
        

        // 15.
        // ABCDE
        // ABCD
        // ABC
        // AB
        // A
        System.out.println("\nPattern 15:");
        pattern_15();


        // 16.
        // A
        // BB
        // CCC
        // DDDD
        // EEEEE
        System.out.println("\nPattern 16:");
        pattern_16();


        // 17.
        //    A
        //   ABA
        //  ABCBA
        // ABCDCBA
        System.out.println("\nPattern 17:");
        pattern_17();


        // 18.
        // E
        // D E
        // C D E
        // B C D E
        // A B C D E
        System.out.println("\nPattern 18:");
        pattern_18();

        // 19.
        // **********
        // ****  ****
        // ***    ***
        // **      **
        // *        *
        // *        *
        // **      **
        // ***    ***
        // ****  ****
        // **********
        System.out.println("\nPattern 19:");
        pattern_19();


        // 20.
        // *        *
        // **      **
        // ***    ***
        // ****  ****
        // **********
        // ****  ****
        // ***    ***
        // **      **
        // *        *
        System.out.println("\nPattern 20:");
        pattern_20();


        // 21.
        // ****
        //
        // *  *
        //
        // *  *
        //
        // ****
        System.out.println("\nPattern 21:");
        pattern_21();

        // 22.
        // 4444444
        // 4333334
        // 4322234
        // 4321234
        // 4322234
        // 4333334
        // 4444444
        System.out.println("\nPattern 22:");
        pattern_22();
    }

    public static void pattern_1() {
        for (int i=0; i<5; i++) {
            for (int j = 0; j < 5; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_2() {
        for (int i = 0; i < 5; i++) {
            for (int j=0; j <= i; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_3() {
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j <= i; j++) System.out.print(j+1);
            System.out.println();
        }
    }

    public static void pattern_4() {
        for (int i=0; i<5; i++) {
            for (int j=0; j<=i; j++) System.out.print(i+1);
            System.out.println();
        }
    }

    public static void pattern_5() {
        for (int i=0; i<5; i++) {
            for (int j=0; j<5-i; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_6() {
        for (int i=0; i<5; i++) {
            for (int j=0; j<5-i; j++) System.out.print(j+1);
            System.out.println();
        }
    }

    public static void pattern_7() {
        for (int i=0; i<5; i++){
            for (int j=1; j<5-i; j++) System.out.print(" ");
            for (int j=0; j<i; j++) System.out.print("*");
            for (int k=0; k<=i; k++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_8() {
        for (int i=0; i<5; i++) {
            for (int j=0; j<i; j++) System.out.print(" ");
            for (int j=0; j<5-i; j++) System.out.print("*");
            for (int j=1; j<5-i; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_9() {
        for (int i=0; i<5; i++) {
            for (int j = 1; j < 5-i; j++) System.out.print(" ");
            for (int j = 0; j <= i; j++) System.out.print("*");
            for (int j = 0; j < i; j++) System.out.print("*");
            System.out.println();
        }
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j < i; j++) System.out.print(" ");
            for (int j = 0; j < 5-i; j++) System.out.print("*");
            for (int j = 1; j < 5-i; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_10() {
        for (int i=0; i<5; i++) {
            for (int j=0; j<=i; j++) System.out.print("*");
            System.out.println();
        }
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j < 5-i; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_11() {
        for (int i=0; i<5; i++) {
            int toggle = (i % 2 == 0) ? 1 : 0;
            for (int j = 0; j <= i; j++) {
                System.out.print(toggle+" ");
                toggle = (toggle == 0) ? 1 : 0;
            }
            System.out.println();
        }
    }

    // hard to think : practice more with such types
    public static void pattern_12() {
        for (int i=0; i<4; i++) {
            for (int j = 0; j <= i; j++) System.out.print(j+1);
            for (int j = 0; j < 4-i-1; j++) System.out.print(" ");
            for (int j = 0; j < 4-i-1; j++) System.out.print(" ");
            for (int j = 0; j <= i; j++) System.out.print(i-j+1);
            System.out.println();
        }
    }

    public static void pattern_13() {
        int counter = 1;
        for (int i=0; i<5; i++) {
            for (int j = 0; j <= i; j++) {
                System.out.print(counter + " ");
                counter++;
            }
            System.out.println();
        }
    }

    public static void pattern_14() {
        for (int i=0; i<5; i++) {
            char ch = 'A';
            for (int j = 0; j <= i; j++) {
                System.out.print(ch);
                ch = (char) (ch + 1);
            }
            System.out.println();
        }
    }

    public static void pattern_15() {
        for (int i=0; i<5; i++) {
            char ch = 'A';
            for ( int j = 0; j < 5-i; j++) {
                System.out.print(ch);
                ch = (char) (ch + 1);
            }
            System.out.println();
        }
    }

    public static void pattern_16() {
        char ch = 'A';
        for (int i=0; i<5; i++) {
            for ( int j = 0; j <= i; j++) {
                System.out.print(ch);
            }
            ch = (char) (ch + 1);
            System.out.println();
        }
    }
    
    // same char is incrementing first and then decrementing:
    public static void pattern_17() {
        for (int i=0; i<4; i++) {
            char ch = 'A';
            for (int j = 0; j < 4-i-1; j++) System.out.print(" ");
            for (int j = 0; j <= i; j++) {
                System.out.print(ch);
                ch = (char) (ch+1);
            }
            ch = (char) (ch-1);
            for (int j = 0; j < i; j++) {
                ch = (char) (ch-1);
                System.out.print(ch);
            }
            System.out.println();
        }
    }

    public static void pattern_18() {
        for (int i = 0; i < 5; i++) {
            char ch = 'A';
            for (int j = 0; j <= i; j++) System.out.print((char) (ch+5-i+j-1) + " ");
            System.out.println();
        }
    }

    public static void pattern_19() {
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j < 5-i; j++) System.out.print("*");
            for (int j = 0; j < i; j++) System.out.print(" ");
            for (int j = 0; j < i; j++) System.out.print(" ");
            for (int j = 0; j < 5-i; j++) System.out.print("*");
            System.out.println();
        }
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j <= i; j++) System.out.print("*");
            for (int j = 0; j < 5-i-1; j++) System.out.print(" ");
            for (int j = 0; j < 5-i-1; j++) System.out.print(" ");
            for (int j = 0; j <= i; j++) System.out.print("*");
            System.out.println();
        }

    }

    public static void pattern_20() {
        for (int i = 0; i < 5; i++) {
            for (int j = 0; j <= i; j++) System.out.print("*");
            for (int j = 0; j < 5-i-1; j++) System.out.print(" ");
            for (int j = 0; j < 5-i-1; j++) System.out.print(" ");
            for (int j = 0; j <= i; j++) System.out.print("*");
            System.out.println();
        }
        for (int i = 0; i < 4; i++) {
            for (int j = 0; j < 5-i-1; j++) System.out.print("*");
            for (int j = 0; j < i+1; j++) System.out.print(" ");
            for (int j = 0; j < i+1; j++) System.out.print(" ");
            for (int j = 0; j < 5-i-1; j++) System.out.print("*");
            System.out.println();
        }
    }

    public static void pattern_21() {
        int n = 5;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if ( i == 0 || j == 0 || i == (n-1) || j == (n-1) ) System.out.print("*");
                else System.out.print(" ");
            }
            System.out.println();
        }
    }


    // NOTE: Important: 
    // We'll calculate the distance of the cell from all ends and will print the value in terms of the minimum distance:
    // let n = 4;
    // 4444444
    // 4333334
    // 4322234
    // 4321234
    // 4322234
    // 4333334
    // 4444444
    // Here, outer layer perimeter is '4' i.e n
    // just one inner layer in perimeter is '3' i.e n-1
    // one layer inner perimeter is '2' i.e n-2
    //
    //NOTE: here the depth will be : 4 3 2 1 2 3 4 => 4321 + 234 => n + (n-1)
    // therefore, for n=4 -> run loop till : n+(n-1) : 7 times
    public static void pattern_22() {
        int n = 4;
        int depth = n + (n-1);
        for (int i = 0; i < depth; i++) {
            for (int j = 0; j < depth; j++) {
                int top = i+1;
                int left = j+1;
                int bottom = depth - i;
                int right = depth - j;

                int num = Math.min(Math.min(top,bottom), Math.min(left,right));
                System.out.print(n-num+1);
            }
            System.out.println();
        }
    }

}
