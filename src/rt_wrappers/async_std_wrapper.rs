use super::{run, AlternateScreenPagingError, PagerMutex};

/// Run the pager inside an [`async_std task`](async_std::task).
///
/// This function is only available when `async_std_lib` feature is enabled
/// It takes a [`PagerMutex`] and updates the page with new information when `PagerMutex`
/// is updated.
///
/// This function switches to the [`Alternate Screen`] of the TTY and switches
/// to [`raw mode`].
///
/// [`Alternate Screen`]: crossterm::terminal#alternate-screen
/// [`raw mode`]: crossterm::terminal#raw-mode
///
/// ## Errors
///
/// Several operations can fail when outputting information to a terminal, see
/// the [`Result`] type.
///
/// ## Example
///
/// ```rust,no_run
/// use async_std::task::sleep;
/// use futures::join;

/// use std::fmt::Write;
/// use std::time::Duration;

/// #[async_std::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///    let output = minus::Pager::new().unwrap().finish();
///
///    let increment = async {
///        for i in 0..=30_u32 {
///            let mut output = output.lock().await;
///            output.push_str(&format!("{}", i));
///            drop(output);
///            sleep(Duration::from_millis(100)).await;
///        }
///        Result::<_, std::fmt::Error>::Ok(())
///    };

///    let (res1, res2) = join!(minus::async_std_updating(output.clone()), increment);
///    res1?;
///    res2?;
///    Ok(())
/// }
/// ```
///
/// **Please do note that you should never lock the output data, since this
/// will cause the paging thread to be paused. Only borrow it when it is
/// required and drop it if you have further asynchronous blocking code.**
#[cfg(feature = "async_std_lib")]
#[cfg_attr(docsrs, doc(cfg(feature = "async_std_lib")))]
pub async fn async_std_updating(pager: PagerMutex) -> Result<(), AlternateScreenPagingError> {
    async_std::task::spawn(run(pager)).await
}
