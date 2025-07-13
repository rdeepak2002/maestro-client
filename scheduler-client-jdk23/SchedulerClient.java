import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

public class SchedulerClient {
    static MethodHandle divNumbers;

    static {
        System.loadLibrary("schedulerclient_jdk23");
        Linker linker = Linker.nativeLinker();
        SymbolLookup lib = SymbolLookup.loaderLookup();
        divNumbers = linker.downcallHandle(
            lib.find("div_numbers").orElseThrow(),
            FunctionDescriptor.of(
                ValueLayout.JAVA_DOUBLE,
                ValueLayout.JAVA_DOUBLE,
                ValueLayout.JAVA_DOUBLE
            )
        );
    }

    public static void main(String[] args)  throws Throwable {
        double result = (double) SchedulerClient.divNumbers.invokeExact(4.0, 2.0);
        assert result == 2;
        System.out.println("4 / 2 = " + result);
    }
}
