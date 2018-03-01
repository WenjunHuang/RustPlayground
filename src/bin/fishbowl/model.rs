use std::default::Default;
use std::collections::HashMap;
use gtk::*;
use gdk::prelude::*;
use gdk::FrameClock;
use glib_sys;


const N_STATS: usize = 5;
const STATS_UPDATE_TIME: i32 = glib_sys::G_USEC_PER_SEC;

#[derive(Debug, Default)]
pub struct Stats {
  last_stats: i64,
  last_frame: i64,
  last_suggestion: i32,
  frame_counter_max: u32,
  stats_index: u32,
  frame_counter: [u32; N_STATS],
  item_counter: [u32; N_STATS],
}

impl Stats {
  pub fn new(widget: Widget) -> Self {
    let n = widget.get_frame_clock().unwrap().get_frame_time();
    Stats {
      last_frame: n,
      last_stats: n,
      ..Default::default()
    }
  }
}

struct WidgetStats {
  widget_stats: HashMap<Widget, Stats>,
}

impl WidgetStats {
  fn new() -> Self {
    WidgetStats {
      widget_stats: HashMap::new(),
    }
  }

  fn get_stats_mut(&mut self, widget: &Widget) -> &mut Stats {
    let f = clone!(widget => move || Stats::new(widget));
    let stats = self.widget_stats
      .entry(widget.clone())
      .or_insert_with(f);
    stats
  }

  fn do_stats(&mut self, widget: &Widget, info_label: &Label) {
    let stats = self.get_stats_mut(widget);
    let frame_time = widget.get_frame_clock().unwrap().get_frame_time();

    if stats.last_stats + STATS_UPDATE_TIME < frame_time {
      let mut n_frames = 0;
      for i in 0..N_STATS {
        n_frames += stats.frame_counter[i];
      }
      let new_label = format!("icons - {} fps",
                              STATS_UPDATE_TIME * n_frames / (N_STATS * STATS_UPDATE_TIME));
      label.set_label(new_label);

      if stats.frame_counter[stats.stats_index] >= 19 * stats.frame_counter_max / 20 {
        if stats.last_suggestion > 0 {
          stats.last_suggestion *= 2;
        } else {
          stats.last_suggestion = 1;
        }
      } else {
        if stats.last_suggestion < 0 {
          stats.last_suggestion -= 1;
        } else {
          stats.last_suggestion = -1;
        }
      }

      stats.stats_index = (stats.stats_index + 1) % N_STATS;
      stats.frame_counter[stats.stats_index] = 0;
      stats.item_counter[stats.stats_index] = stats.item_counter[(stats.stats_index + N_STATS - 1) % N_STATS];
      stats.last_stats = frame_time;
    }

    stats.last_frame = frame_time;
    stats.frame_counter[stats.stats_index] += 1;
    stats.frame_counter_max = stats.frame_counter_max.max(stats.frame_counter[stats.stats_index]);
  }

  fn stats_update(&mut self, widget: &Widget) {
    let stats = self.get_stats_mut(widget.clone());
    stats.item_counter[stats.stats_index]
  }
}

struct FishBowlChild {
  widget:Widget,
  x:f64,
  y:f64,
  dx:f64,
  dy:f64,
}

struct FishBowl {
  container: Container,
  children: Vec<FishBowlChild>,
  last_frame_time: i64,
  tick_id: u32,
}

impl FishBowl {
  fn new() -> Self {
    let container = Container::new();
    container.set_has_window(false);
    FishBowl {
      container: container,
    }
  }
}