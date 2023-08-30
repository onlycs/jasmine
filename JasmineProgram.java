
import java.util.*;
import java.util.stream.*;

public class JasmineProgram {

	public static void print(String formatstr, Object... args) {
		System.out.printf(formatstr.replace("{}", "%s"), args);
	}

	public static void println(String formatstr, Object... args) {
		print(formatstr + "\n", args);
	}

	public static String format(String formatstr, Object... args) {
		return String.format(formatstr.replace("{}", "%s"), args);
	}

	public static class Option<T> {
		public static final int _Some = 1;
		public static final int _None = 2;
		int currentVariant;
		T SomeData;

		private Option(int _currentVariant, T _SomeData) {
			this.currentVariant = _currentVariant;
			this.SomeData = _SomeData;
		}

		public static <T> Option<T> Some(T data) {
			return new Option<T>(_Some, data);
		}

		public static <T> Option<T> None() {
			return new Option<T>(_None, null);
		}

		public T _getData_Some() {
			return SomeData;
		}

		public boolean is(int variant) {
			return currentVariant == variant;
		}

		public boolean isSome() {
			if (this.is(Option._Some)) {
				return true;
			} else if (this.is(Option._None)) {
				return false;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public boolean isSomeAnd(Closure_Generic_RetBoolean<T> function) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return function.call(val);
			} else if (this.is(Option._None)) {
				return false;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public boolean isNone() {
			return !this.isSome();
		}

		public T unwrap() {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return val;
			} else if (this.is(Option._None)) {
				throw new RuntimeException("called `Option::unwrap()` on a `None` value");
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public T unwrapOr(T def) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return val;
			} else if (this.is(Option._None)) {
				return def;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public T unwrapOrElse(Closure_void_RetGeneric<T> function) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return val;
			} else if (this.is(Option._None)) {
				return function.call();
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> Option<U> map(Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return Option.Some(function.call(val));
			} else if (this.is(Option._None)) {
				return Option.None();
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> U mapOr(U default_value, Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return function.call(val);
			} else if (this.is(Option._None)) {
				return default_value;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> U mapOrElse(Closure_void_RetGeneric<U> default_value,
				Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return function.call(val);
			} else if (this.is(Option._None)) {
				return default_value.call();
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> T expect(U err) {
			if (this.is(Option._Some)) {
				T val = this._getData_Some();
				return val;
			} else if (this.is(Option._None)) {
				throw new RuntimeException(err.toString());
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}
	}


	public static class Result<T, E> {
		public static final int _Ok = 1;
		public static final int _Err = 2;
		int currentVariant;
		T OkData;
		E ErrData;

		private Result(int _currentVariant, T _OkData, E _ErrData) {
			this.currentVariant = _currentVariant;
			this.OkData = _OkData;
			this.ErrData = _ErrData;
		}

		public static <T, E> Result<T, E> Ok(T data) {
			return new Result<T, E>(_Ok, data, null);
		}

		public static <T, E> Result<T, E> Err(E data) {
			return new Result<T, E>(_Err, null, data);
		}

		public T _getData_Ok() {
			return OkData;
		}

		public E _getData_Err() {
			return ErrData;
		}

		public boolean is(int variant) {
			return currentVariant == variant;
		}

		public boolean isOk() {
			if (this.is(Result._Ok)) {
				return true;
			} else {
				return false;
			}
		}

		public boolean isOkAnd(Closure_Generic_RetBoolean<T> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return function.call(val);
			} else if (this.is(Result._Err)) {
				return false;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public boolean isErr() {
			return !this.isOk();
		}

		public boolean isErrAnd(Closure_Generic_RetBoolean<E> function) {
			if (this.is(Result._Err)) {
				E val = this._getData_Err();
				return function.call(val);
			} else if (this.is(Result._Ok)) {
				return false;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public T unwrap() {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return val;
			} else if (this.is(Result._Err)) {
				E err = this._getData_Err();
				throw new RuntimeException(err.toString());
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public T unwrapOr(T def) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return val;
			} else {
				return def;
			}
		}

		public T unwrapOrElse(Closure_void_RetGeneric<T> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return val;
			} else {
				return function.call();
			}
		}

		public Option<T> ok() {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return Option.Some(val);
			} else {
				return Option.None();
			}
		}

		public Option<E> err() {
			if (this.is(Result._Err)) {
				E val = this._getData_Err();
				return Option.Some(val);
			} else {
				return Option.None();
			}
		}

		public <U> Result<U, E> map(Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return Result.Ok(function.call(val));
			} else if (this.is(Result._Err)) {
				E val = this._getData_Err();
				return Result.Err(val);
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> U mapOr(U default_value, Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return function.call(val);
			} else if (this.is(Result._Err)) {
				return default_value;
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> U mapOrElse(Closure_void_RetGeneric<U> default_value,
				Closure_Generic_RetGeneric<T, U> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return function.call(val);
			} else if (this.is(Result._Err)) {
				return default_value.call();
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}

		public <U> Result<T, U> mapErr(Closure_Generic_RetGeneric<E, U> function) {
			if (this.is(Result._Ok)) {
				T val = this._getData_Ok();
				return Result.Ok(val);
			} else if (this.is(Result._Err)) {
				E val = this._getData_Err();
				return Result.Err(function.call(val));
			} else {
				throw new RuntimeException("Not all match arms were covered in this statement");
			}
		}
	}

	public static class Vec<T> implements Iterable<T> {
		ArrayList<T> inner;

		public Iterator<T> iterator() {
			return this.inner.iterator();
		}

		public Vec() {
			this.inner = new ArrayList<T>();
		}

		public static <T> Vec<T> from(T... items) {
			Vec<T> vec = new Vec<T>();

			for (T item : items) {
				vec.inner.add(item);
			}

			return vec;
		}

		public void push(T item) {
			this.inner.add(item);
		}

		public Option<T> pop() {
			if (this.inner.size() == 0) {
				return Option.None();
			} else {
				return Option.Some(this.inner.remove(this.inner.size() - 1));
			}
		}

		public Option<T> get(int index) {
			if (index < 0 || index >= this.inner.size()) {
				return Option.None();
			} else {
				return Option.Some(this.inner.get(index));
			}
		}

		public void insert(int index, T item) {
			this.inner.add(index, item);
		}

		public void remove(int index) {
			this.inner.remove(index);
		}

		public int len() {
			return this.inner.size();
		}

		public void retain(Closure_Generic_RetBoolean<T> function) {
			for (int i = 0; i < this.len(); i++) {
				if (!function.call(this.get(i).unwrap())) {
					this.inner.remove(i);
					i--;
				}
			}
		}

		public void append(Vec<T> other) {
			for (T item : other) {
				this.inner.add(item);
			}
		}

		public void clear() {
			this.inner.clear();
		}

		public boolean isEmpty() {
			return this.inner.isEmpty();
		}

		public void sortBy(Closure_Generic_RetInteger<T> function) {
			Collections.sort(this.inner, new Comparator<T>() {
				public int compare(T a, T b) {
					return function.call(a) - function.call(b);
				}
			});
		}

		public void extend(Vec<T> other) {
			other.append(this);
			this.inner = other.inner;
		}

		public Vec<T> clone() {
			Vec<T> vec = new Vec<T>();
			vec.inner = (ArrayList<T>) this.inner.clone();
			return vec;
		}

		public Double sumFloat() {
			return this.inner.stream().mapToDouble(x -> (Double) x).sum(); // java creators, where clause wen
		}

		public Integer sumInt() {
			return this.inner.stream().mapToInt(x -> (Integer) x).sum();
		}

		public Double sum() {
			return this.sumFloat();
		}

		public Double average() {
			return this.sumFloat() / this.len();
		}

		public Stream<T> stream() {
			return this.inner.stream();
		}
	}

	public static class Range implements Iterable<Integer> {
		int start;
		int end;
		boolean inclusive;

		public Range(int start, int end, boolean inclusive) {
			this.start = start;
			this.end = end;
			this.inclusive = inclusive;
		}

		public Iterator<Integer> iterator() {
			return new Iterator<Integer>() {
				int current = start;

				public boolean hasNext() {
					if (inclusive) {
						return current <= end;
					} else {
						return current < end;
					}
				}

				public Integer next() {
					int val = current;
					current++;
					return val;
				}
			};
		}
	}

	public interface Closure_Generic_RetBoolean<Arg0> {
		Boolean call(Arg0 arg0);
	}
	public interface Closure_Generic_RetGeneric<Arg0, Ret> {
		Ret call(Arg0 arg0);
	}
	public interface Closure_void_RetGeneric<Ret> {
		Ret call();
	}
	public interface Closure_Generic_RetInteger<Arg0> {
		Integer call(Arg0 arg0);
	}

	public static void main(String[] args) {
		for (Integer i : new Range(0, 10, true)) {
			println("{}", i);
		}
	}
}
