import { Err } from "../error/err.ts";
import { Position } from "../error/position.ts";
import { RuntimeError } from "../error/runtimeerr.ts";
import { Context } from "./context.ts";

export class Number {
  public positionStart!: Position | null;
  public positionEnd!: Position | null;
  public context!: Context | null;
  constructor(public value: number) {
    this.setPosition();
    this.setContext();
  }

  public setPosition(
    start: Position | null = null,
    end: Position | null = null,
  ) {
    this.positionStart = start;
    this.positionEnd = end;
    return this;
  }

  public addTo(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value + other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public subBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value - other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public multiBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      return {
        result: new Number(this.value * other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public divBy(
    other: Number,
  ): { result: Number | null; error: Err | null } | undefined {
    if (other instanceof Number) {
      if (other.value == 0) {
        return {
          result: null,
          error: new RuntimeError(
            other.positionStart!,
            other.positionEnd!,
            "Division by 0 isn't possible",
            this.context!,
          ),
        };
      }
      return {
        result: new Number(this.value / other.value).setContext(this.context),
        error: null,
      };
    }
  }

  public setContext(context: Context | null = null) {
    this.context = context;
    return this;
  }

  public represent() {
    return `${this.value}`;
  }
}
