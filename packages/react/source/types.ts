/** A Palette for a Theme */
export interface ITheme<Tokens, Assets> {
	/** Name of the theme i.e. 'default' or 'christmas' */
	name: string;
	/** Design tokens of the theme - an object with your properties */
	tokens?: Tokens;
	/** Assets of the theme - an object with your assets i.e. logos */
	assets?: Assets;
};

export interface IParameters<Props, Propless, Themes> {
	readonly base?: string;
	readonly themes?: Partial<{ [Key in keyof Themes]: Themes[Key] extends string | undefined ? Partial<{ [Value in Extract<Themes[Key], string>]: string }> : string }>;
	readonly variants?: Propless extends null ? Partial<{ [Key in keyof Props]: Props[Key] extends string | undefined ? Partial<{ [Value in Extract<Props[Key], string>]: string }> : string }> : Propless
	readonly compounds?: Array<[Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean }>, string]>,
};

export type TmixArgs<Props, Propless, Tokens, Assets, Themes> = ((theme?: ITheme<Tokens, Assets>) => IParameters<Props, Propless, Themes>);

export type TmixFunction<Tokens, Assets, Themes> = <Props = null, Propless = null>(arg: TmixArgs<Props, Propless, Tokens, Assets, Themes>) => (props?: Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean; }>) => string;

export type Tpalette<Tokens, Assets, Themes> = {
	createMix: (themeIndex?: number) => TmixFunction<Tokens, Assets, Themes>;

	palette: ITheme<Tokens, Assets>[]
};