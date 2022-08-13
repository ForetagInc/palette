/** A Palette for a Theme */
export interface IPalette<Tokens, Assets> {
	/** Name of the theme i.e. 'default' or 'christmas' */
	name: string;
	/** Design tokens of the theme - an object with your properties */
	tokens?: Tokens;
	/** Assets of the theme - an object with your assets i.e. logos */
	assets?: Assets;
};

export interface IParameters<Props, Propless> {
	readonly base?: string;
	readonly variants?: Propless extends null ? Partial<{ [Key in keyof Props]: Props[Key] extends string | undefined ? Partial<{ [Value in Extract<Props[Key], string>]: string }> : string }> : Propless
	readonly compounds?: Array<[Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean }>, string]>,
};

export type TmixArgs<X, Y, T, A> = ((theme?: IPalette<T, A>) => IParameters<X, Y>) | IParameters<X, Y>;
export type TmixFunction<T, A> = <Props = null, Propless = null>(arg: TmixArgs<Props, Propless, T, A>) => (props?: Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean; }>) => string;
export type Tpalette<T, A> = { mix: TmixFunction<T, A>, palette: IPalette<T, A>[] };