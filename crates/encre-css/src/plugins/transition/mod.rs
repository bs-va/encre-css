//! Transition and animation utilities
pub mod animation;
pub mod transition_delay;
pub mod transition_duration;
pub mod transition_property;
pub mod transition_timing_function;

#[cfg(test)]
mod tests {
    use crate::utils::testing;

    use pretty_assertions::assert_eq;

    #[test]
    fn animation() {
        assert_eq!(
            testing::generate_css("animate-none"),
            ".animate-none {
  -webkit-animation: none;
  animation: none;
}"
        );
        assert_eq!(
            testing::generate_css("animate-spin"),
            "@-webkit-keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  -webkit-animation: spin 1s linear infinite;
  animation: spin 1s linear infinite;
}"
        );
        assert_eq!(
            testing::generate_css("animate-ping"),
            "@-webkit-keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

.animate-ping {
  -webkit-animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
  animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
}"
        );
        assert_eq!(
            testing::generate_css("animate-pulse"),
            "@-webkit-keyframes pulse {
  50% {
    opacity: .5;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: .5;
  }
}

.animate-pulse {
  -webkit-animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}"
        );
        assert_eq!(
            testing::generate_css("animate-bounce"),
            "@-webkit-keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }

  50% {
    transform: none;
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }
  50% {
    transform: translateY(0);
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }
}

.animate-bounce {
  -webkit-animation: bounce 1s infinite;
  animation: bounce 1s infinite;
}"
        );
    }

    #[test]
    fn transition_delay() {
        assert_eq!(
            testing::generate_css("delay-12"),
            ".delay-12 {
  transition-delay: 12ms;
}"
        );
        assert_eq!(
            testing::generate_css("delay-[4.3ms]"),
            r".delay-\[4\.3ms\] {
  transition-delay: 4.3ms;
}"
        );
    }

    #[test]
    fn transition_duration() {
        assert_eq!(
            testing::generate_css("duration-12"),
            ".duration-12 {
  transition-duration: 12ms;
}"
        );
        assert_eq!(
            testing::generate_css("duration-[4.3ms]"),
            r".duration-\[4\.3ms\] {
  transition-duration: 4.3ms;
}"
        );
    }

    #[test]
    fn transition_property() {
        assert_eq!(
            testing::generate_css("transition"),
            ".transition {
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-none"),
            ".transition-none {
  transition-property: none;
}"
        );
        assert_eq!(
            testing::generate_css("transition-all"),
            ".transition-all {
  transition-property: all;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-colors"),
            ".transition-colors {
  transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-opacity"),
            ".transition-opacity {
  transition-property: opacity;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-shadow"),
            ".transition-shadow {
  transition-property: box-shadow;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-transform"),
            ".transition-transform {
  transition-property: transform;
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
  transition-duration: 150ms;
}"
        );
        assert_eq!(
            testing::generate_css("transition-[custom]"),
            r".transition-\[custom\] {
  transition-property: custom;
}"
        );
    }

    #[test]
    fn transition_timing_function() {
        assert_eq!(
            testing::generate_css("ease-linear"),
            ".ease-linear {
  transition-timing-function: linear;
}"
        );
        assert_eq!(
            testing::generate_css("ease-in"),
            ".ease-in {
  transition-timing-function: cubic-bezier(0.4, 0, 1, 1);
}"
        );
        assert_eq!(
            testing::generate_css("ease-out"),
            ".ease-out {
  transition-timing-function: cubic-bezier(0, 0, 0.2, 1);
}"
        );
        assert_eq!(
            testing::generate_css("ease-in-out"),
            ".ease-in-out {
  transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
}"
        );
        assert_eq!(
            testing::generate_css("ease-[steps(4,_jump-end)]"),
            r".ease-\[steps\(4\,_jump-end\)\] {
  transition-timing-function: steps(4, jump-end);
}"
        );
    }
}
