    use super::super::*;

    #[test]
    fn should_return_audio_file_name() {
        let result = get_audio_file_name("abc", &PronunciationRegion::Us);
        assert_eq!(result, "abc_us.mp3");
    }
