<script lang="ts">
	import { onMount } from 'svelte';

	import Section from '../lib/section.svelte';

	import beach from '$lib/assets/beach.png?w=1920&format=webp';
	import golf from '$lib/assets/golf.jpg?w=1920&format=webp';
	import tennis from '$lib/assets/tennis.jpg?w=1920&format=webp';
	import sailing from '$lib/assets/sailing.jpg?w=1920&format=webp';
	import sunbathing from '$lib/assets/sunbathing.jpg?w=1920&format=webp';
	import modernismo from '$lib/assets/modernismo.jpg?w=1920&format=webp';
	import caleta from '$lib/assets/caleta.jpg?w=1920&format=webp';
	import beachclub from '$lib/assets/beachclub_1.jpg?w=1920&format=webp';

	var data = [
		{ name: 'Tennis', events: [] },
		{ name: 'Golf', events: [] }
	];
	var lang = 'DE';

	function format_date(date_str: string): string {
		var date = new Date(date_str);
		let hours = date.getHours();
		return `${hours}${hours >= 12 ? 'pm' : 'am'}`;
	}

	onMount(async () => {
		const recaptchaScript = document.createElement('script');
		recaptchaScript.setAttribute('src', 'https://assets.calendly.com/assets/external/widget.js');
		document.head.appendChild(recaptchaScript);

		const res = await fetch(`/api/events`);
		data = await res.json();
		console.log(data);
	});
</script>

<div class="mx-8 my-4 flex flex-row space-x-4 text-xl">
	<label>
		<input type="radio" bind:group={lang} name="scoops" value={'DE'} class="apperance-none" />
		Deutsch
	</label>
	<label>
		<input type="radio" bind:group={lang} name="scoops" value={'ES'} />
		Español
	</label>
</div>

<Section background={beach} dir="left">
	<div slot="title" class="font-bold">
		BEACHPARTY <br />
		15-17 SEPT 2023 <br />
		SANT ANDREU DE LLAVANERES <br />
	</div>
	<div slot="content" class="flex flex-col align-start md:uppercase">
		<span class="font-bold my-4 text-amber-100"
			>{lang == 'DE' ? 'Freitag September 15' : 'Viernes Septiembre 15'}</span
		>
		<div class="grid grid-cols-2 grid-cols-[85px_auto] place-content-start gap-x-8">
			<span class="text-right text-amber-100">19:30</span>
			<span class="">
				{lang == 'DE' ? 'Drinks und Tapas in Can Markus' : 'Tapeo y copasn en Can Markus'}
			</span>
		</div>
		<span class="font-bold my-4 text-amber-100"
			>{lang == 'DE' ? 'Samstag September 16' : 'Sabado Septiembre 16'}</span
		>
		<div class="grid grid-cols-2 grid-cols-[85px_auto] place-content-start gap-x-8">
			<span class="text-right text-amber-100">9h - 19h</span>
			<span class="">
				{lang == 'DE'
					? 'Golf / Tennis / Segeln / Sonnen / Spaziergang / Altersgerechte gymnastik'
					: 'Golf / Penis / Vela / Panching / Paseo modernista / Gimnasia playera'}
			</span>
			<span class="text-right text-amber-100">14:00</span>
			<span> Watering hole & Wine spot, La Caleta </span>
			<span class="text-right text-amber-100">15:00</span>
			<span>
				{lang == 'DE' ? 'Lange Siesta' : 'Siesta profunda'}
			</span>
			<span class="text-right text-amber-100">20:00</span>
			<span>Beachparty {lang == 'DE' ? 'im' : 'en'} GAS Quiet Club</span>
		</div>
		<span class="font-bold my-4 text-amber-100"
			>{lang == 'DE' ? 'Sonntag September 17' : 'Domingo Septiembre 17'}</span
		>
		<div class="grid grid-cols-2 grid-cols-[85px_auto] place-content-start gap-x-8">
			<span class="text-right text-amber-100">10:30</span>
			<span class="">Café & Croissant, Petit Moll</span>
		</div>
	</div>
</Section>

<Section background={golf}>
	<div slot="title">
		GOLF <br />
		9:30 / 9:40 / 9:50
	</div>
	<div slot="content">
		{lang == 'DE'
			? 'Leider nur für Frühaufsteher. Abschlag bei Tee 10.'
			: 'Solo para los madrugadores. Salida en el tee 10.'}
		<br />
		{lang == 'DE'
			? 'Dank noch vorhanden Einladungen (60 EUR) von Markus können wir die Greenfee (200 EUR) wahrscheinlich senken. Bitte Golfschläger selber mitnehmen.'
			: 'Gracias a las invitaciones aún disponibles (60 EUR) de Markus se reduce probablemente el green fee (200 EUR). Por favor, traigan sus propios palos de golf.'}
		<br />

		Location:
		<a target="_blank" href="https://goo.gl/maps/oBozjG8pFH6khkGS7" class="hover:underline">
			Club de Golf de Llavaneres, Camí Golf, 49-51, Sant Andreu de Llavaneres
		</a>
	</div>
</Section>

<Section background={tennis} dir="left">
	<div slot="title">
		{lang == 'DE' ? 'Tennis / Padel' : 'Tenis / Padel'}
		<br />
		12:00 / 13:00
	</div>
	<div slot="content">
		{lang == 'DE' ? 'Einzel oder Doppel.' : 'Individual o dobles.'}
		<br />
		Location:
		<a target="_blank" href="https://goo.gl/maps/XWrdD2A6qd4DWhCR8" class="hover:underline">
			Tenis Mora, Camí de Can Pi, Sant Andreu de Llavaneres
		</a>

		<br />
	</div>
</Section>

<Section background={sailing} dir="right">
	<div slot="title">
		{lang == 'DE' ? 'Segeln' : 'Vela'}
		<br />
		12:00 - 14:00
	</div>
	<div slot="content">
		Location:
		<a target="_blank" href="https://goo.gl/maps/bNpxA4junTPRhK5y9" class="hover:underline">
			Club Náutico El Balís, Sant Andreu de Llavaneres.
		</a>
		{lang == 'DE' ? 'Liegeplatz' : 'Amarre'}: 468 - 470
	</div>
</Section>

<Section background={sunbathing} dir="left" gradient={false}>
	<div slot="title" class="text-slate-800">
		{lang == "DE" ? "Sonnen" : "Panching"}<br />
		12:00 - 18:00
	</div>
	<div slot="content" class="text-black">
		{lang == 'DE'
			? 'Vor Chiringuito Ohnades, bei schlechtem Wetter am Swimmingpool von Can Markus. '
			: 'Delante del chiringuito Ohnades, con mal tiempo en la piscina de Can Markus'}
		<br />
		Locations: <br />
		<a target="_blank" href="https://goo.gl/maps/74dJ5GVgj3zkKXnb8" class="hover:underline">
			Chiringuito Ohnades, Passeig del Marquès de Casa Riera, 25, 08394 Sant Vicenç de Montalt
		</a>
		<br />
		<a target="_blank" href="https://goo.gl/maps/hPoaACdjNvqbLUT4A" class="hover:underline">
			“Can Markus”, Avinguda Turo d’en Llull 64, 08392 Sant Andreu de Llavaneres
		</a>
	</div>
</Section>

<Section background={modernismo} dir="right">
	<div slot="title">
		{lang == 'DE' ? 'Spaziergang' : 'Paseo moderista'}
		<br />
		12:00
	</div>
	<div slot="content">
		{lang == 'DE' ? 'Spaziergang mit katalanischem Modernismus in' : 'En'}
		<a target="_blank" href="https://goo.gl/maps/USMcxD9bHwfiHV3b9" class="hover:underline">
			Canet de Mar.
		</a>
	</div>
</Section>

<Section background={caleta} dir="left">
	<div slot="title">
		Watering hole & Wine spot <br />
		14:00
	</div>
	<div slot="content">
		Location:
		<a target="_blank" href="https://goo.gl/maps/RDwLUgaAk2XUwCx5A" class="hover:underline">
			La Caleta, Passeig del Marquès de Casa Riera, 45, Sant Vicenç de Montalt
		</a>
		<br/>
		<span class="font-bold my-2">
			{lang == 'DE' ? 'Mittagessen' : 'Comida: '} <br />
		</span>
		{lang == 'DE' ? 'Unsere Vorschläge:' : 'Nuestras propuestas:'}
		La Caleta, Ohnades, Mío, Sotavent, Club de Golf.
		{lang == 'DE'
			? 'Sagt bitte Bescheid, wenn wir für Euch vorreservieren sollen.'
			: 'Por favor avisarnos si queréis que reservemos.'}
		<br />
	</div>
</Section>

<Section background={beachclub} dir="right">
	<div slot="title">
		Beachparty <br />
		20:00
	</div>
	<div slot="content">
		{lang == 'DE' ? 'Im GAS Quiet Club.' : 'En el GAS Quiet Club.'}
		<br />
		{lang == 'DE'
			? 'Dress code: Sommerlich mit Sand-festen Schuhen oder Barfuss.'
			: 'Dress code: Veraniego con zapatos a prueba de arena o descalzos.'}
		<br />
		Location:
		<a target="_blank" href="https://goo.gl/maps/QVKbsvKvxKM4yViw9" class="hover:underline">
			GAS Quiet Club, Club Náutico El Balís, Sant Andreu de Llavaneres
		</a>
	</div>
</Section>

<div class="my-24 mx-6 md:mx-40 flex flex-col space-y-6">
	<h2 class="text-4xl font-display">
		{lang == 'DE' ? 'Anreise' : 'Llegada'}
	</h2>
	<div class="mx-6 md:mx-40">
		{lang == 'DE'
			? 'Wenn Ihr möchtet, gebt Eure Flüge und Ankunftszeiten an, um gemeinsam Autos auszuleihen oder per Sammeltaxi nach Llavaneres zu fahren.'
			: 'Si queréis, darnos vuestros vuelos o trenes y hora de llegada (ver más abajo), para organizar el alquiler de coches o de coger un taxi grande a Llavaneres.'}
		<br />
		{lang == 'DE'
			? 'Per Taxi: Kristian Taxi, 6 Sitze: ... €, 4 Sitze: ... € Edu Taxi, 4 Sitze: ...'
			: 'Por Taxi: Kristian Taxi, 6 plazas: ... €, 4 plazas: ... € Edu Taxi, 4 plazas: ...'}
	</div>
</div>

<div class="my-24 mx-6 md:mx-40 flex flex-col space-y-6">
	<h2 class="text-4xl font-display">
		{lang == 'DE' ? 'Semmelsches Partytraining' : 'Calentando motores a lo Semmel'}
	</h2>
	{#each data as event_type}
		<div class="mx-6 md:mx-40">
			<span class="text-xl font-bold">{event_type.name}</span>
			<div class="grid grid-cols-2 grid-cols-[60px_auto] gap-x-4">
				{#each event_type.events as event}
					<span class="text-right"> {format_date(event['start'])}:</span>
					<div class="">
						{event['attendees'].join(', ')}
					</div>
				{/each}
			</div>
		</div>
	{/each}
	<!-- Calendly inline widget begin -->
	<div
		class="calendly-inline-widget min-w-[320px] h-[700px]"
		data-url="https://calendly.com/semmelbeachparty?hide_gdpr_banner=0"
	/>
	<!-- Calendly inline widget end -->
</div>

<div class="my-24 mx-6 md:mx-40 flex flex-col space-y-6">
	<h2 class="text-4xl font-display">
		{lang == 'DE' ? 'Adressen' : 'Direcciones'}
	</h2>
	<div class="mx-6 md:mx-40">
		“Can Markus”, Avinguda Turo d’en Llull 64, 08392 Sant Andreu de Llavaneres <br />
		Club de Golf de Llavaneres, Camí Golf, 49-51, Sant Andreu de Llavaneres <br />
		Tenis Mora, Camí de Can Pi, Sant Andreu de Llavaneres <br />
		Club Náutico El Balís, Sant Andreu de Llavaneres, Amarres 468 – 470 <br />
		Chiringuito Ohnades, Passeig del Marquès de Casa Riera, 25, 08394 Sant Vicenç de Montalt <br />
		La Caleta, Passeig del Marquès de Casa Riera, 45, Sant Vicenç de Montalt, Tel. +34 937 911 558
		<br />
		Ohnades, Passeig del Marquès de Casa Riera, 25, Sant Vicenç de Montalt, Tel: +34 692 336 653
		<br />
		Mío, Passeig del Marquès de Casa Riera, 1, Sant Vicenç de Montalt, +34 605 511 413 <br />
		Sotavent, Passeig dels Anglesos, 24, 08393 Caldes d'Estrac, +34 661 759 445 <br />
		GAS Quiet Club, Club Náutico El Balís, Sant Andreu de Llavaneres <br />
		Petit Moll, Passeig del Marquès de Casa Riera, Sant Vicenç de Montalt <br />
	</div>
</div>

<div class="my-24 mx-6 md:mx-40 flex flex-col space-y-6">
	<h2 class="text-4xl font-display">
		{lang == 'DE' ? 'Telefone' : 'Móviles'}
	</h2>
	<div class="mx-6 md:mx-40">
		Markus: 629 129636 <br />
		Nata: 619 818 229 <br />
		Miriam: +34 638 089 540 <br />
		Frederik: +41 788 544 271 (WhatsApp) <br />
	</div>
</div>
