
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```sh
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const ALLUSERSPROFILE: string;
	export const APPDATA: string;
	export const CMD_DURATION_MS: string;
	export const COLORTERM: string;
	export const CommonProgramFiles: string;
	export const CommonProgramW6432: string;
	export const COMPUTERNAME: string;
	export const ComSpec: string;
	export const DriverData: string;
	export const EFC_15908_1592913036: string;
	export const EFC_15908_344590478: string;
	export const GoLand: string;
	export const GOPATH: string;
	export const HOME: string;
	export const HOMEDRIVE: string;
	export const HOMEPATH: string;
	export const INIT_CWD: string;
	export const LANG: string;
	export const LAST_EXIT_CODE: string;
	export const LOCALAPPDATA: string;
	export const LOGONSERVER: string;
	export const NODE: string;
	export const NODE_ENV: string;
	export const NODE_PATH: string;
	export const npm_command: string;
	export const npm_config_frozen_lockfile: string;
	export const npm_config_node_gyp: string;
	export const npm_config_recursive: string;
	export const npm_config_registry: string;
	export const npm_config_user_agent: string;
	export const npm_config_verify_deps_before_run: string;
	export const npm_execpath: string;
	export const npm_lifecycle_event: string;
	export const npm_lifecycle_script: string;
	export const npm_node_execpath: string;
	export const npm_package_name: string;
	export const npm_package_version: string;
	export const NUMBER_OF_PROCESSORS: string;
	export const NU_VERSION: string;
	export const NVM_HOME: string;
	export const NVM_SYMLINK: string;
	export const OLDPWD: string;
	export const OneDrive: string;
	export const OS: string;
	export const Path: string;
	export const PATHEXT: string;
	export const PNPM_HOME: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const POSH_INSTALLER: string;
	export const POSH_SESSION_ID: string;
	export const POSH_SHELL: string;
	export const POSH_SHELL_VERSION: string;
	export const POSH_THEME: string;
	export const POSH_THEMES_PATH: string;
	export const POWERLINE_COMMAND: string;
	export const PROCESSOR_ARCHITECTURE: string;
	export const PROCESSOR_IDENTIFIER: string;
	export const PROCESSOR_LEVEL: string;
	export const PROCESSOR_REVISION: string;
	export const ProgramData: string;
	export const ProgramFiles: string;
	export const ProgramW6432: string;
	export const PROMPT: string;
	export const PROMPT_INDICATOR: string;
	export const PROMPT_INDICATOR_VI_INSERT: string;
	export const PROMPT_INDICATOR_VI_NORMAL: string;
	export const PROMPT_MULTILINE_INDICATOR: string;
	export const PSModulePath: string;
	export const PUBLIC: string;
	export const PWD: string;
	export const PyCharm: string;
	export const RustRover: string;
	export const SESSIONNAME: string;
	export const SHLVL: string;
	export const SystemDrive: string;
	export const SystemRoot: string;
	export const TEMP: string;
	export const TERM: string;
	export const TERM_PROGRAM: string;
	export const TERM_PROGRAM_VERSION: string;
	export const TMP: string;
	export const TRANSIENT_PROMPT_COMMAND_RIGHT: string;
	export const TRANSIENT_PROMPT_MULTILINE_INDICATOR: string;
	export const USERDOMAIN: string;
	export const USERDOMAIN_ROAMINGPROFILE: string;
	export const USERNAME: string;
	export const USERPROFILE: string;
	export const WebStorm: string;
	export const windir: string;
	export const ZED_TERM: string;
	export const ZES_ENABLE_SYSMAN: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > [!NOTE] In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		ALLUSERSPROFILE: string;
		APPDATA: string;
		CMD_DURATION_MS: string;
		COLORTERM: string;
		CommonProgramFiles: string;
		CommonProgramW6432: string;
		COMPUTERNAME: string;
		ComSpec: string;
		DriverData: string;
		EFC_15908_1592913036: string;
		EFC_15908_344590478: string;
		GoLand: string;
		GOPATH: string;
		HOME: string;
		HOMEDRIVE: string;
		HOMEPATH: string;
		INIT_CWD: string;
		LANG: string;
		LAST_EXIT_CODE: string;
		LOCALAPPDATA: string;
		LOGONSERVER: string;
		NODE: string;
		NODE_ENV: string;
		NODE_PATH: string;
		npm_command: string;
		npm_config_frozen_lockfile: string;
		npm_config_node_gyp: string;
		npm_config_recursive: string;
		npm_config_registry: string;
		npm_config_user_agent: string;
		npm_config_verify_deps_before_run: string;
		npm_execpath: string;
		npm_lifecycle_event: string;
		npm_lifecycle_script: string;
		npm_node_execpath: string;
		npm_package_name: string;
		npm_package_version: string;
		NUMBER_OF_PROCESSORS: string;
		NU_VERSION: string;
		NVM_HOME: string;
		NVM_SYMLINK: string;
		OLDPWD: string;
		OneDrive: string;
		OS: string;
		Path: string;
		PATHEXT: string;
		PNPM_HOME: string;
		PNPM_SCRIPT_SRC_DIR: string;
		POSH_INSTALLER: string;
		POSH_SESSION_ID: string;
		POSH_SHELL: string;
		POSH_SHELL_VERSION: string;
		POSH_THEME: string;
		POSH_THEMES_PATH: string;
		POWERLINE_COMMAND: string;
		PROCESSOR_ARCHITECTURE: string;
		PROCESSOR_IDENTIFIER: string;
		PROCESSOR_LEVEL: string;
		PROCESSOR_REVISION: string;
		ProgramData: string;
		ProgramFiles: string;
		ProgramW6432: string;
		PROMPT: string;
		PROMPT_INDICATOR: string;
		PROMPT_INDICATOR_VI_INSERT: string;
		PROMPT_INDICATOR_VI_NORMAL: string;
		PROMPT_MULTILINE_INDICATOR: string;
		PSModulePath: string;
		PUBLIC: string;
		PWD: string;
		PyCharm: string;
		RustRover: string;
		SESSIONNAME: string;
		SHLVL: string;
		SystemDrive: string;
		SystemRoot: string;
		TEMP: string;
		TERM: string;
		TERM_PROGRAM: string;
		TERM_PROGRAM_VERSION: string;
		TMP: string;
		TRANSIENT_PROMPT_COMMAND_RIGHT: string;
		TRANSIENT_PROMPT_MULTILINE_INDICATOR: string;
		USERDOMAIN: string;
		USERDOMAIN_ROAMINGPROFILE: string;
		USERNAME: string;
		USERPROFILE: string;
		WebStorm: string;
		windir: string;
		ZED_TERM: string;
		ZES_ENABLE_SYSMAN: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
