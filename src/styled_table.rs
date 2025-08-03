use comfy_table::Table;
use comfy_table::TableComponent::BottomBorder;
use comfy_table::TableComponent::BottomBorderIntersections;
use comfy_table::TableComponent::BottomLeftCorner;
use comfy_table::TableComponent::BottomRightCorner;
use comfy_table::TableComponent::HeaderLines;
use comfy_table::TableComponent::HorizontalLines;
use comfy_table::TableComponent::LeftBorder;
use comfy_table::TableComponent::LeftBorderIntersections;
use comfy_table::TableComponent::LeftHeaderIntersection;
use comfy_table::TableComponent::MiddleHeaderIntersections;
use comfy_table::TableComponent::MiddleIntersections;
use comfy_table::TableComponent::RightBorder;
use comfy_table::TableComponent::RightBorderIntersections;
use comfy_table::TableComponent::RightHeaderIntersection;
use comfy_table::TableComponent::TopBorder;
use comfy_table::TableComponent::TopBorderIntersections;
use comfy_table::TableComponent::TopLeftCorner;
use comfy_table::TableComponent::TopRightCorner;
use comfy_table::TableComponent::VerticalLines;

/// A styled Table that is highly inspired by DuckDB's output format.
pub(crate) fn styled_table() -> Table {
    let mut table = Table::new();
    // corner
    table.set_style(TopLeftCorner, '┌');
    table.set_style(TopRightCorner, '┐');
    table.set_style(BottomLeftCorner, '└');
    table.set_style(BottomRightCorner, '┘');

    // intersections
    table.set_style(TopBorderIntersections, '┬');
    table.set_style(BottomBorderIntersections, '┴');
    table.set_style(LeftBorderIntersections, '├');
    table.set_style(RightBorderIntersections, '┤');
    table.set_style(LeftHeaderIntersection, '├');
    table.set_style(RightHeaderIntersection, '┤');
    table.set_style(MiddleHeaderIntersections, '┼');
    table.set_style(MiddleIntersections, '┼');

    // lines
    table.set_style(HeaderLines, '─');
    table.set_style(HorizontalLines, '─');
    table.set_style(VerticalLines, '│');

    // border
    table.set_style(TopBorder, '─');
    table.set_style(BottomBorder, '─');
    table.set_style(LeftBorder, '│');
    table.set_style(RightBorder, '│');

    table
}
