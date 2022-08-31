/** A Palette for a Theme */
export interface IPalette<Tokens, Assets> {
	/** Name of the theme i.e. 'default' or 'christmas' */
	name: string;
	/** Design tokens of the theme - an object with your properties */
	tokens?: Tokens;
	/** Assets of the theme - an object with your assets i.e. logos */
	assets?: Assets;
};

export interface IParameters<Props, Propless, Themes> {
	readonly base?: string;
	readonly themes?: Themes;
	readonly variants?: Propless extends null ? Partial<{ [Key in keyof Props]: Props[Key] extends string | undefined ? Partial<{ [Value in Extract<Props[Key], string>]: string }> : string }> : Propless
	readonly compounds?: Array<[Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean }>, string]>,
};

export type TmixArgs<X, Y, Tokens, Assets, Themes> = ((theme?: IPalette<Tokens, Assets>) => IParameters<X, Y, Themes>) | IParameters<X, Y, Themes>;

export type TmixFunction<Tokens, Assets, Themes> = <Props = null, Propless = null>(arg: TmixArgs<Props, Propless, Tokens, Assets, Themes>) => (props?: Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean; }>) => string;

export type Tpalette<Tokens, Assets, Themes> = {
	createMix: (themeIndex?: number) => <Props = null, Propless = null>(arg: TmixArgs<Props, Propless, Tokens, Assets, Themes>) => (props?: Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean; }>) => string
	palette: IPalette<Tokens, Assets>[]
};