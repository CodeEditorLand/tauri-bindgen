interface Tauri {
  invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
}
declare global {
  interface Window {
    __TAURI__: { tauri: Tauri };
  }
}
const { invoke } = window.__TAURI__.tauri;
export type E1 = "a";
export type U1 = U10 | U11;
export interface U10 {
  tag: 0;
  val: number;
}
export interface U11 {
  tag: 1;
  val: number;
}
export interface Empty {}
export type V1 = V1A | V1B | V1C | V1D | V1E | V1F | V1G;
export interface V1A {
  tag: "a";
}
export interface V1B {
  tag: "b";
  val: U1;
}
export interface V1C {
  tag: "c";
  val: E1;
}
export interface V1D {
  tag: "d";
  val: string;
}
export interface V1E {
  tag: "e";
  val: Empty;
}
export interface V1F {
  tag: "f";
}
export interface V1G {
  tag: "g";
  val: number;
}
export type Casts1 = Casts1A | Casts1B;
export interface Casts1A {
  tag: "a";
  val: number;
}
export interface Casts1B {
  tag: "b";
  val: number;
}
export type Casts2 = Casts2A | Casts2B;
export interface Casts2A {
  tag: "a";
  val: number;
}
export interface Casts2B {
  tag: "b";
  val: number;
}
export type Casts3 = Casts3A | Casts3B;
export interface Casts3A {
  tag: "a";
  val: number;
}
export interface Casts3B {
  tag: "b";
  val: bigint;
}
export type Casts4 = Casts4A | Casts4B;
export interface Casts4A {
  tag: "a";
  val: number;
}
export interface Casts4B {
  tag: "b";
  val: bigint;
}
export type Casts5 = Casts5A | Casts5B;
export interface Casts5A {
  tag: "a";
  val: number;
}
export interface Casts5B {
  tag: "b";
  val: bigint;
}
export type Casts6 = Casts6A | Casts6B;
export interface Casts6A {
  tag: "a";
  val: [number, number];
}
export interface Casts6B {
  tag: "b";
  val: [number, number];
}
export type MyErrno = "bad1" | "bad2";
export interface IsClone {
  v1: V1;
}
export async function e1Arg(x: E1): Promise<void> {
  await invoke<void>("plugin:imports|e1_arg", { x: x });
}
export async function e1Result(): Promise<E1> {
  const result = await invoke<E1>("plugin:imports|e1_result");
  return result;
}
export async function u1Arg(x: U1): Promise<void> {
  await invoke<void>("plugin:imports|u1_arg", { x: x });
}
export async function u1Result(): Promise<U1> {
  const result = await invoke<U1>("plugin:imports|u1_result");
  return result;
}
export async function v1Arg(x: V1): Promise<void> {
  await invoke<void>("plugin:imports|v1_arg", { x: x });
}
export async function v1Result(): Promise<V1> {
  const result = await invoke<V1>("plugin:imports|v1_result");
  return result;
}
export async function boolArg(x: boolean): Promise<void> {
  await invoke<void>("plugin:imports|bool_arg", { x: x });
}
export async function boolResult(): Promise<boolean> {
  const result = await invoke<boolean>("plugin:imports|bool_result");
  return result;
}
export async function optionArg(
  a: boolean | null,
  b: [] | null,
  c: number | null,
  d: E1 | null,
  e: number | null,
  f: U1 | null,
  g: Option<boolean | null>
): Promise<void> {
  await invoke<void>("plugin:imports|option_arg", {
    a: a,
    b: b,
    c: c,
    d: d,
    e: e,
    f: f,
    g: g,
  });
}
export async function optionResult(): Promise<
  [
    boolean | null,
    [] | null,
    number | null,
    E1 | null,
    number | null,
    U1 | null,
    Option<boolean | null>
  ]
> {
  const result = await invoke<
    [
      boolean | null,
      [] | null,
      number | null,
      E1 | null,
      number | null,
      U1 | null,
      Option<boolean | null>
    ]
  >("plugin:imports|option_result");
  return result;
}
export async function casts(
  a: Casts1,
  b: Casts2,
  c: Casts3,
  d: Casts4,
  e: Casts5,
  f: Casts6
): Promise<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]> {
  const result = await invoke<[Casts1, Casts2, Casts3, Casts4, Casts5, Casts6]>(
    "plugin:imports|casts",
    { a: a, b: b, c: c, d: d, e: e, f: f }
  );
  return result;
}
export async function resultArg(
  a: Result<void, void>,
  b: Result<void, E1>,
  c: Result<E1, void>,
  d: Result<[], []>,
  e: Result<number, V1>,
  f: Result<string, Uint8Array>
): Promise<void> {
  await invoke<void>("plugin:imports|result_arg", {
    a: a,
    b: b,
    c: c,
    d: d,
    e: e,
    f: f,
  });
}
export async function resultResult(): Promise<
  [
    Result<void, void>,
    Result<void, E1>,
    Result<E1, void>,
    Result<[], []>,
    Result<number, V1>,
    Result<string, Uint8Array>
  ]
> {
  const result = await invoke<
    [
      Result<void, void>,
      Result<void, E1>,
      Result<E1, void>,
      Result<[], []>,
      Result<number, V1>,
      Result<string, Uint8Array>
    ]
  >("plugin:imports|result_result");
  return result;
}
export async function returnResultSugar(): Promise<number> {
  const result = await invoke<number>("plugin:imports|return_result_sugar");
  return result;
}
export async function returnResultSugar2(): Promise<void> {
  const result = await invoke<void>("plugin:imports|return_result_sugar2");
  return result;
}
export async function returnResultSugar3(): Promise<MyErrno> {
  const result = await invoke<MyErrno>("plugin:imports|return_result_sugar3");
  return result;
}
export async function returnResultSugar4(): Promise<[number, number]> {
  const result = await invoke<[number, number]>(
    "plugin:imports|return_result_sugar4"
  );
  return result;
}
export async function returnOptionSugar(): Promise<number | null> {
  const result = await invoke<number | null>(
    "plugin:imports|return_option_sugar"
  );
  return result;
}
export async function returnOptionSugar2(): Promise<MyErrno | null> {
  const result = await invoke<MyErrno | null>(
    "plugin:imports|return_option_sugar2"
  );
  return result;
}
export async function resultSimple(): Promise<number> {
  const result = await invoke<number>("plugin:imports|result_simple");
  return result;
}
export async function isCloneArg(a: IsClone): Promise<void> {
  await invoke<void>("plugin:imports|is_clone_arg", { a: a });
}
export async function isCloneReturn(): Promise<IsClone> {
  const result = await invoke<IsClone>("plugin:imports|is_clone_return");
  return result;
}
export async function returnNamedOption(): Promise<number | null> {
  const result = await invoke<number | null>(
    "plugin:imports|return_named_option"
  );
  return result;
}
export async function returnNamedResult(): Promise<number> {
  const result = await invoke<number>("plugin:imports|return_named_result");
  return result;
}
export type Option<T> = { tag: "none" } | { tag: "some"; val; T };
export type Result<T, E> = { tag: "ok"; val: T } | { tag: "err"; val: E };