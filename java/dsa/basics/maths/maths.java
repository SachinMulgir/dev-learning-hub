package java.dsa.basics.maths;

import java.util.Scanner;

public class maths {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        // GCF and HCF
        System.out.println("Enter your numbers:");
        int n1 = sc.nextInt();
        int n2 = sc.nextInt();

        hcf(n1, n2);


        // Prime Number:
        System.out.println("\n\nEnter number to check for prime:");
        int num = sc.nextInt();

        System.out.println("Is " + num + " prime: " + is_prime(num));
        sc.close();
    }

    public static void hcf(int a, int b) {
        int min = (a < b) ? a : b;
        int hcf = 0;
        System.out.print("Common divisor for " + a + " & " + b + " are:");
        for (int i = 1; i <= min; i++) {
            if ( a % i == 0 && b % i == 0) {
                System.out.print(" " + i + ",");
                hcf = (hcf > i) ? hcf : i;
            }
        }
        System.out.println("\nThe HCF for " + a + " & " + b + " is: " + hcf);
    }

    public static boolean is_prime(int num) {
        int count = 0;
        for (int i = 2; i < num; i++) {
            if (num % i == 0) count++;
        }
        return (count > 0) ? false : true;
    }
}