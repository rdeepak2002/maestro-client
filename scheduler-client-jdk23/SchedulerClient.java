import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.VarHandle;
import java.util.UUID;

class Workflow implements AutoCloseable {
    private final MemorySegment nativePtr;
    private final Arena arena;

    // todo: this should be private
    public Workflow(MemorySegment nativePtr, Arena arena) {
        this.nativePtr = nativePtr;
        this.arena = arena;
    }

    public String getId() {
        try {
            MemorySegment idPtr = (MemorySegment) SchedulerClient.workflowGetId.invokeExact(nativePtr);
            if (idPtr.equals(MemorySegment.NULL)) {
                return null;
            }
            String id = idPtr.reinterpret(Long.MAX_VALUE).getString(0);
            SchedulerClient.freeString.invokeExact(idPtr);
            return id;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public static WorkflowBuilder builder() {
        return new WorkflowBuilder();
    }

    @Override
    public void close() {
        try {
            SchedulerClient.workflowFree.invokeExact(nativePtr);
            arena.close();
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}

class WorkflowBuilder {
    private MemorySegment builderPtr;
    private final Arena arena;

    public WorkflowBuilder() {
        try {
            this.arena = Arena.ofConfined();
            this.builderPtr = (MemorySegment) SchedulerClient.workflowBuilderNew.invokeExact();
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public WorkflowBuilder id(String id) {
        try {
            MemorySegment idSegment = arena.allocateFrom(id);
            this.builderPtr = (MemorySegment) SchedulerClient.workflowBuilderSetId.invokeExact(builderPtr, idSegment);
            return this;
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }

    public Workflow build() {
        try {
            MemorySegment workflowPtr = (MemorySegment) SchedulerClient.workflowBuilderBuild.invokeExact(builderPtr);
            return new Workflow(workflowPtr, arena);
        } catch (Throwable e) {
            throw new RuntimeException(e);
        }
    }
}

public class SchedulerClient {
    static MethodHandle workflowBuilderNew;
    static MethodHandle workflowBuilderSetId;
    static MethodHandle workflowBuilderBuild;
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
