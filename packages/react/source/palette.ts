import type {
	ITheme,
	TmixArgs,
	Tpalette,
} from './types';

export const createPalette = <Tokens, Assets>(palette?: ITheme<Tokens, Assets>[]): Tpalette<Tokens, Assets> => {
	const createMix = (themeIndex = 0) => {
		return <Props = null, Propless = null>(arg: TmixArgs<Props, Propless, Tokens, Assets>) => {
			const { base, variants, compounds } = arg(palette[themeIndex]);

			return (props?: Partial<Propless extends null ? Props : { [Key in keyof Propless]: Propless[Key] extends object ? keyof Propless[Key] : boolean }>) => {
				const classes = base ? [...base.split(' ')] : [];

				if (props) {
					if (variants) {
						let variantKeys = Object.keys(props);
						let variantsLength = variantKeys.length;
						for (let i = 0; i < variantsLength; i++) {
							const [key, value] = [variantKeys[i], props[variantKeys[i]]];
							const vari: unknown = variants as unknown;

							if (typeof variants[key as keyof typeof variants] === 'string' && value === true)
								classes.push(...(variants[key as keyof typeof variants] as string).split(' '));
							else if (typeof variants[key as keyof typeof variants] === 'object'
								&& variants[key as keyof typeof variants][value as keyof { [key in keyof typeof vari]: typeof variants[key] }])
								classes.push(...(variants[key as keyof typeof variants][value as keyof { [key in keyof typeof vari]: typeof variants[key] }] as string).split(' '));
						};
					}

					if (compounds) {
						const compoundsLength = compounds.length;

						for (let i = 0; i < compoundsLength; i++) {
							const entries = Object.entries(compounds[i][0]);
							let matches = 0;

							entries.forEach(([key, value]) => {
								props[key as keyof typeof props] === value && matches++;
							});

							if (entries.length === matches) {
								classes.push(...compounds[i][1].split(' '));
							}
						}
					}
				}

				return classes.join(' ').trim();
			}
		};
	}

	return {
		createMix,
		palette
	}
};