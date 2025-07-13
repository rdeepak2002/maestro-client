import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.util.UUID;

class Workflow extends NativeObject {
    Workflow(MemorySegment nativePtr, Arena arena) {
        super(nativePtr, arena);
    }

    public String getId() {
        return getNativeString(SchedulerClient.workflowGetId);
    }

    public static WorkflowBuilder builder() {
        return new WorkflowBuilder();
    }

    @Override
    protected MethodHandle getNativeDestructor() {
        return SchedulerClient.workflowFree;
    }
}

class WorkflowBuilder extends NativeObject {
    private WorkflowBuilder(MemorySegment nativePtr, Arena arena) {
        super(nativePtr, arena);
    }

    public WorkflowBuilder() {
        this(createBuilder(), Arena.ofConfined());
    }

    private static MemorySegment createBuilder() {
        try {
            return (MemorySegment) SchedulerClient.workflowBuilderNew.invokeExact();
        } catch (Throwable e) {
            throw new RuntimeException("Failed to create WorkflowBuilder", e);
        }
    }

    public WorkflowBuilder id(String id) {
        checkNotClosed();
        try {
            MemorySegment idSegment = arena.allocateFrom(id);
            MemorySegment newPtr = (MemorySegment) SchedulerClient.workflowBuilderSetId.invokeExact(nativePtr, idSegment);
            // The builder pattern in Rust returns the same object with modifications
            if (!newPtr.equals(nativePtr)) {
                throw new RuntimeException("Unexpected pointer change in id setter");
            }
            return this;
        } catch (Throwable e) {
            throw new RuntimeException("Failed to set id", e);
        }
    }

    public Workflow build() {
        return consumeAndCreate(
            SchedulerClient.workflowBuilderBuild,
            Workflow::new
        );
    }

    @Override
    protected MethodHandle getNativeDestructor() {
        // WorkflowBuilder doesn't need explicit destruction as it's consumed by build()
        // But we need to provide something for the base class
        return SchedulerClient.workflowBuilderFree;
    }
}

public class SchedulerClient {
    static MethodHandle workflowBuilderNew;
    static MethodHandle workflowBuilderSetId;
    static MethodHandle workflowBuilderBuild;
    static MethodHandle workflowBuilderFree;
    static MethodHandle workflowGetId;
    static MethodHandle workflowFree;
    static MethodHandle freeString;

    static {
        System.loadLibrary("schedulerclient_jdk23");
        Linker linker = Linker.nativeLinker();
        SymbolLookup lib = SymbolLookup.loaderLookup();
        
        workflowBuilderNew = linker.downcallHandle(
            lib.find("workflow_builder_new").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS)
        );
        
        workflowBuilderSetId = linker.downcallHandle(
            lib.find("workflow_builder_set_id").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS)
        );
        
        workflowBuilderBuild = linker.downcallHandle(
            lib.find("workflow_builder_build").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS)
        );
        
        workflowBuilderFree = linker.downcallHandle(
            lib.find("workflow_builder_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)
        );
        
        workflowGetId = linker.downcallHandle(
            lib.find("workflow_get_id").orElseThrow(),
            FunctionDescriptor.of(ValueLayout.ADDRESS, ValueLayout.ADDRESS)
        );
        
        workflowFree = linker.downcallHandle(
            lib.find("workflow_free").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)
        );
        
        freeString = linker.downcallHandle(
            lib.find("free_string").orElseThrow(),
            FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)
        );
    }

    public static void main(String[] args) {
        Workflow actual = Workflow.builder().id("x").build();
        assert "x".equals(actual.getId());
        System.out.println("Java tests pass");
    }
}
