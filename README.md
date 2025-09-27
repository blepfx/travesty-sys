# travesty-sys
`travesty-sys` is a Rust port of DPF's [travesty](https://github.com/DISTRHO/DPF/tree/main/distrho/src/travesty), a pure C VST3-compatible interface.

## Licensing

> [!WARNING]
> The author of this project is not a lawyer and has a limited understanding of all the intricacies of software licensing and copyright. **Use this project at your own risk!** This section is subject to change. Please message me if you found a mistake in the licensing section.

This project mostly based on [travesty](https://github.com/DISTRHO/DPF/tree/main/distrho/src/travesty), and therefore might be considered a derivative work of **travesty**, which is licensed under ISC, a permissive MIT-compatible license. The license notice for **travesty** is [included in this repo](LICENSE-travesty.md).

The original **travesty** bindings themselves may or may not be considered a derivative work of VST3SDK based on how they were produced, [but according to **travesty** authors](https://github.com/DISTRHO/DPF/blob/main/LICENSING.md#vst3-special-note):
> Contrary to most plugins, DPF does not use the official VST3 SDK.
> Instead, the API definitions are provided by the travesty sub-project, licensed in the same way as DPF. **This allows us to freely build plugins without being encumbered by restrictive licensing deals**.
> It makes the internal implementation much harder for DPF, but this is not an issue for external developers.

It's highly likely that using **travesty** or **travesty-sys** would not free you from the distribution and licensing limitations of Steinberg's VST3 dual-license agreement, but using **travesty** or **travesty-sys** _might have_ the advantage of being able to redistribute the bindings and projects that use said bindings under a more permissive license than GPLv3, allowing them to be used in non-GPLv3 (i.e. Proprietary Steinberg VST 3 license) plugins. This makes it easier to distribute VST3 bindings for other languages without having to resort to distributing [bindings _generators_](https://github.com/coupler-rs/vst3-rs) or [GPLv3 licensed bindings](https://github.com/RustAudio/vst3-sys).
