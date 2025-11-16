package DSA.Integers;

import java.util.Scanner;

public class palindrome {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int num = sc.nextInt();
        System.out.println("Palindrome : " + isPalindrome(num));

        sc.close();
    }
    public static boolean isPalindrome(int x) {
        int temp = x;
        int ans = 0;
        while (x > 0) {
            ans = (ans * 10) + (x % 10);
            x = x / 10;
        }

        return (ans == temp);
    }
}
