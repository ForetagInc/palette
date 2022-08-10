import { describe, it, expect } from 'bun:test';
import { createPalette } from '@/source';

describe('Handling assets', () => {
	const { palette } = createPalette([
		{
			name: 'screen',
			assets: {
				logo: 'https://foret.ag/logo.png',
			},
			tokens: {
				radius: {
					default: 4
				}
			}
		},
		{
			name: 'xr',
			assets: {
				icon: 'https://foret.ag/icon.png',
			}
		}
	]);

	it('Should return the right assets', () => {
		expect(palette[0].assets.logo).toBe('https://foret.ag/logo.png');
	});
});