// AUTO-GENERATED by typescript-type-def

export default types;
export namespace types{
export type HTMLElement=("Div"|"P"|"Pre"|"Ul"|"Ol"|"Li"|"A");
export type Command=({"CreateElement":{"el":types.HTMLElement;"parent":(string|null);"attrs":(Record<string,string>|null);};}|{"RemoveElement":{"id":string;};});
export type Response=({"CreatedOk":{"id":string;};}|{"CreatedError":{"message":string;};}|"RemovedOk"|{"RemovedError":{"message":string;};});
}
