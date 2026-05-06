#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_video_copy() {
        let mut builder = FfmpegBuilder::new();
        let input_idx = builder.add_input("input.mp4");

        builder.map(input_idx, "v:0?");
        builder.codec("v:0", "copy");

        let args = builder.build("mp4", "output.mp4");

        assert_eq!(args[0], "-y");

        let expected_subset = vec![
            "-i".to_string(),
            "input.mp4".to_string(),
            "-map".to_string(),
            "0:v:0?".to_string(),
            "-c:v:0".to_string(),
            "copy".to_string()
        ];

        let args_str = args.join(" ");
        assert!(args_str.contains(&expected_subset.join(" ")));
    }
}
#[derive(Default)]
pub struct FfmpegBuilder {
    global_flags: Vec<String>,
    inputs: Vec<String>,
    output_args: Vec<String>,
}

impl FfmpegBuilder {
    pub fn new() -> Self {
        Self {
            global_flags: vec![
                "-y".into(),
                "-hide_banner".into(),
                "-hwaccel".into(),
                "auto".into()
            ],
            ..Default::default()
        }
    }

    pub fn add_input(&mut self, path: &str) -> usize {
        if let Some(idx) = self.inputs.iter().position(|p| p == path) {
            return idx;
        }
        self.inputs.push(path.into());
        self.inputs.len() - 1
    }

    pub fn map(&mut self, input_idx: usize, stream_specifier: &str) -> &mut Self {
        self.output_args.push("-map".into());
        self.output_args.push(format!("{}:{}", input_idx, stream_specifier));
        self
    }

    pub fn codec(&mut self, specifier: &str, codec: &str) -> &mut Self {
        self.output_args.push(format!("-c:{}", specifier));
        self.output_args.push(codec.into());
        self
    }

    pub fn filter(&mut self, specifier: &str, filter: &str) -> &mut Self {
        self.output_args.push(format!("-filter:{}", specifier));
        self.output_args.push(filter.into());
        self
    }

    pub fn metadata(&mut self, specifier: &str, key: &str, value: &str) -> &mut Self {
        self.output_args.push(format!("-metadata:s:{}", specifier));
        self.output_args.push(format!("{}={}", key, value));
        self
    }

    pub fn disposition(&mut self, specifier: &str, value: &str) -> &mut Self {
        self.output_args.push(format!("-disposition:{}", specifier));
        self.output_args.push(value.into());
        self
    }

    pub fn arg(&mut self, flag: &str, value: &str) -> &mut Self {
        self.output_args.push(flag.into());
        self.output_args.push(value.into());
        self
    }

    pub fn build(self, muxer: &str, output_path: &str) -> Vec<String> {
        let mut final_args = self.global_flags;

        for input in self.inputs {
            final_args.push("-i".into());
            final_args.push(input);
        }

        final_args.extend(self.output_args);

        final_args.extend(
            vec![
                "-stats".into(),
                "-v".into(),
                "info".into(),
                "-f".into(),
                muxer.into(),
                output_path.into()
            ]
        );

        final_args
    }
}
