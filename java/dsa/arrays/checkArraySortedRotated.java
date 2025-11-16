class checkArraySortedRotated {
    public static void main(String[] args) {
        int[] arr = {3,4,5,1,2};
        check(arr);       
    }
    public static boolean check(int[] nums) {
        for (int i = 0; i < nums.length; i++) {
            boolean check = true;
            for (int j = 0; j < nums.length-1; j++) {
                System.out.println(nums[(i+j)%nums.length] + " " + nums[(i+j+1)%nums.length]);
                if (nums[(i+j)%nums.length] > nums[(i+j+1)%nums.length]) {
                    check=false;
                    break;
                }
            }
            if (check) return true;
            System.out.println(check + "\n\n");
        }
        return false;
}

}

