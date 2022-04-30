#!/usr/bin/env -S deno run --allow-net --allow-run

import {
	DOMParser,
	Element,
} from 'https://deno.land/x/deno_dom@v0.1.22-alpha/deno-dom-wasm.ts';
import { unzip } from 'https://cdn.skypack.dev/lodash';
import { parse } from 'https://deno.land/std@0.137.0/flags/mod.ts';
import { blue, rgb24 } from 'https://deno.land/std@0.137.0/fmt/colors.ts';

const parsed = parse(Deno.args);

const main = async (username?: string) => {
	const url = `https://github.com/${username}`;
	const request = await fetch(url);
	const doc = new DOMParser().parseFromString(
		await request.text(),
		'text/html',
	)!;
	const root = doc.querySelector('g')!;

	if (root === null) {
		console.log(`"${username}" not found`);
		return;
	}

	const elements = root.querySelectorAll('g')!;

	console.log(parsed['colour'] === false ? url : blue(url));

	const months = [];

	for (let i = 0; i < elements.length; i++) {
		const rects = (elements[i] as Element).querySelectorAll('rect');
		const days = [];

		for (let j = 0; j < rects.length; j++) {
			const e = rects[j] as Element;
			const level = e.getAttribute('data-level') as string;

			// Long confusing chains
			const hex = level === '0'
				? 0x161b22
				: level === '1'
				? 0x0e4429
				: level === '2'
				? 0x006d32
				: level === '3'
				? 0x26a641
				: level === '4'
				? 0x39d353
				: 0;

			const character = level === '0'
				? ' '
				: level === '1'
				? '.'
				: level === '2'
				? '+'
				: level === '3'
				? '#'
				: level === '4'
				? '%'
				: ' ';

			if (parsed['colour'] === false) days.push(character);
			else if (parsed['level']) days.push(level);
			else days.push(rgb24('■', hex));
		}

		months.push(days);
	}

	const unziped = unzip(months);

	const cols = Deno.run({
		cmd: ['tput', 'cols'],
		stderr: 'piped',
		stdout: 'piped',
	});
	const output = Number(new TextDecoder().decode(await cols.output()));
	cols.close();

	for (let i = 0; i < unziped.length; i++) {
		// dont overengineer that stuff.. just do confusing chains
		const day = i === 1 ? 'Mon' : i === 3 ? 'Wed' : i === 5 ? 'Fri' : '   ';

		console.log(`${day} ${unziped[i].join(output <= 110 ? '' : ' ')}`);
	}
};

const help = () => {
	// Im not doing this right, right?
	console.log(`View your github contribution calander in unicode.

Usage:
    gh-cal [--no-colour] [--levels]
    gh-cal <username> [--no-colour] [--levels]
    gh-cal -h | --help
`);
};

if (parsed.help || parsed.h) {
	help();
} else {
	let username = parsed._[0];

	if (!username) {
		let output: {
			login?: string;
		};
		try {
			const process = Deno.run({
				cmd: ['gh', 'api', 'user'],
				stderr: 'piped',
				stdout: 'piped',
			});
			output = JSON.parse(
				new TextDecoder().decode(await process.output()),
			);
			process.close();
		} catch (err) {
			output = {};
		}

		if (output.login) username = output.login;
		else {
			console.log(`Provide a username:
Usage:
    gh-cal [username] [--no-colour] [--levels]`);
		}
	}

	username && main(username as string);
}
