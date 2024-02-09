
public class ArgOrder {
    static String newArg(String arg) {
        System.out.println("newArg: " + arg);
        return arg;
    }

    static void consumeArgs(String v1, String v2) {
        System.out.println("consumeArgs: " + v1 + " " + v2);
    }

    public static void main(String[] args) {
        var v1 = newArg("v1");
        var v2 = newArg("v2");
        consumeArgs(v1, v2);
        // newArg: v1
        // newArg: v2
        // consumeArgs: v1 v2

        consumeArgs(newArg("v1"), newArg("v2"));
        // newArg: v1
        // newArg: v2
        // consumeArgs: v1 v2
    }
}
