import java.lang.foreign.*;
import java.lang.invoke.MethodHandle;

/**
 * Base class for Java objects that wrap native Rust objects.
 * Handles memory management and provides common utilities.
 */
public abstract class NativeObject implements AutoCloseable {
    protected final MemorySegment nativePtr;
    protected final Arena arena;
    private boolean closed = false;

    protected NativeObject(MemorySegment nativePtr, Arena arena) {
        this.nativePtr = nativePtr;
        this.arena = arena;
    }

    protected NativeObject(MemorySegment nativePtr) {
        this(nativePtr, Arena.ofConfined());
    }

    /**
     * Creates a new native object by calling the provided constructor function
     */
    protected static <T extends NativeObject> T createNative(
            MethodHandle constructor, 
            NativeObjectFactory<T> factory) throws Throwable {
        Arena arena = Arena.ofConfined();
        MemorySegment ptr = (MemorySegment) constructor.invokeExact();
        return factory.create(ptr, arena);
    }

    /**
     * Calls a native method that returns a string and handles memory cleanup
     */
    protected String getNativeString(MethodHandle getter) {
        checkNotClosed();
        try {
            MemorySegment stringPtr = (MemorySegment) getter.invokeExact(nativePtr);
            if (stringPtr.equals(MemorySegment.NULL)) {
                return null;
            }
            String result = stringPtr.reinterpret(Long.MAX_VALUE).getString(0);
            SchedulerClient.freeString.invokeExact(stringPtr);
            return result;
        } catch (Throwable e) {
            throw new RuntimeException("Failed to get native string", e);
        }
    }

    /**
     * Calls a native method that takes a string parameter and returns self
     */
    protected void setNativeString(MethodHandle setter, String value) {
        checkNotClosed();
        try {
            MemorySegment valueSegment = arena.allocateFrom(value);
            MemorySegment newPtr = (MemorySegment) setter.invokeExact(nativePtr, valueSegment);
            // Note: The native pointer should be the same for in-place modifications
            if (!newPtr.equals(nativePtr)) {
                throw new RuntimeException("Unexpected pointer change in setter");
            }
        } catch (Throwable e) {
            throw new RuntimeException("Failed to set native string", e);
        }
    }

    /**
     * Calls a native method that consumes self and returns a new object
     */
    protected <T extends NativeObject> T consumeAndCreate(
            MethodHandle method, 
            NativeObjectFactory<T> factory) {
        checkNotClosed();
        try {
            MemorySegment newPtr = (MemorySegment) method.invokeExact(nativePtr);
            closed = true; // Mark this object as consumed
            return factory.create(newPtr, arena);
        } catch (Throwable e) {
            throw new RuntimeException("Failed to consume and create object", e);
        }
    }

    protected void checkNotClosed() {
        if (closed) {
            throw new IllegalStateException("Native object has been closed or consumed");
        }
    }

    @Override
    public void close() {
        if (!closed) {
            try {
                getNativeDestructor().invokeExact(nativePtr);
                arena.close();
                closed = true;
            } catch (Throwable e) {
                throw new RuntimeException("Failed to free native object", e);
            }
        }
    }

    /**
     * Subclasses must provide their specific destructor method handle
     */
    protected abstract MethodHandle getNativeDestructor();

    /**
     * Factory interface for creating native objects
     */
    @FunctionalInterface
    public interface NativeObjectFactory<T extends NativeObject> {
        T create(MemorySegment ptr, Arena arena);
    }
}