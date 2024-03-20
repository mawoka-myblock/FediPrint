export enum Licenses {
	CcPd = 'CcPd',
	CcAttr = 'CcAttr',
	CcAttrSa = 'CcAttrSa',
	CcAttrNd = 'CcAttrNd',
	CcAttrNc = 'CcAttrNc',
	CcAttrNcSa = 'CcAttrNcSa',
	CcAttrNcNd = 'CcAttrNcNd',
	Gpl2 = 'Gpl2',
	Gpl3 = 'Gpl3',
	GnuLesser = 'GnuLesser',
	Bsd = 'Bsd',
	Sdfl = 'Sdfl'
}
export const name_to_license: { name: string; value: Licenses }[] = [
	{
		name: 'Creative Commons — Public Domain',
		value: Licenses.CcPd
	},
	{
		name: 'Creative Commons — Attribution',
		value: Licenses.CcAttr
	},
	{
		name: 'Creative Commons — Attribution  — Share Alike',
		value: Licenses.CcAttrSa
	},
	{
		name: 'Creative Commons — Attribution — NoDerivatives',
		value: Licenses.CcAttrNd
	},
	{
		name: 'Creative Commons — Attribution  — Noncommercial',
		value: Licenses.CcAttrNc
	},
	{
		name: 'Creative Commons — Attribution  — Noncommercial  —  Share Alike',
		value: Licenses.CcAttrNcSa
	},
	{
		name: 'Creative Commons — Attribution  — Noncommercial  —  NoDerivatives',
		value: Licenses.CcAttrNcNd
	},
	{
		name: 'GNU General Public License v2.0',
		value: Licenses.Gpl2
	},
	{
		name: 'GNU General Public License v3.0',
		value: Licenses.Gpl3
	},
	{
		name: 'GNU Lesser General Public License',
		value: Licenses.GnuLesser
	},
	{
		name: 'BSD License',
		value: Licenses.Bsd
	},
	{
		name: 'Standard Digital File License',
		value: Licenses.Sdfl
	}
];
