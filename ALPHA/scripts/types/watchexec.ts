export type WatchexecEvent = {
	tags: WatchexecTag[];
	metadata: Record<string, unknown>;
};

export type WatchexecTag =
	| PathTag
	| FsTag
	| SourceTag
	| KeyboardTag
	| ProcessTag
	| SignalTag
	| CompletionTag;

export interface PathTag {
	kind: "path";
	absolute: string;
	filetype?: "dir" | "file" | "symlink" | "other";
}

export interface FsTag {
	kind: "fs";
	simple: "access" | "create" | "modify" | "remove" | "other";
	full: string;
}

export interface SourceTag {
	kind: "source";
	source: "filesystem" | "keyboard" | "mouse" | "os" | "time" | "internal";
}

export interface KeyboardTag {
	kind: "keyboard";
	keycode: "eof";
}

export interface ProcessTag {
	kind: "process";
	pid: number;
}

export interface SignalTag {
	kind: "signal";
	signal: "hangup" | "interrupt" | "quit" | "terminate" | "user1" | "user2";
}

export interface CompletionTag {
	kind: "completion";
	disposition:
		| "success"
		| "error"
		| "signal"
		| "stop"
		| "exception"
		| "continued";
	code: number;
}
