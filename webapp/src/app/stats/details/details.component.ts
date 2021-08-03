import {AfterViewInit, Component, OnDestroy, OnInit, ViewChild} from '@angular/core';
import {RunsQuery} from "../../runs/state/runs.query";
import {RunsService} from "../../runs/state/runs.service";

import * as am4core from "@amcharts/amcharts4/core";
import * as am4charts from "@amcharts/amcharts4/charts";
import am4themes_animated from "@amcharts/amcharts4/themes/animated";
import {UntilDestroy, untilDestroyed} from "@ngneat/until-destroy";
import {MatOption} from "@angular/material/core";
import {MatSelect} from "@angular/material/select";
import {filter, map, startWith, switchMap} from "rxjs/operators";
import {MatSelectChange} from "@angular/material/select/select";

interface DateRange {
  label: string;
  startDate: number; // timestamp in sec

}

@UntilDestroy()
@Component({
  selector: 'app-details',
  templateUrl: './details.component.html',
  styleUrls: ['./details.component.scss']
})
export class DetailsComponent implements OnInit, AfterViewInit, OnDestroy {

  readonly dateRanges: DateRange[] = [];
  selectedDateRange: DateRange;

  @ViewChild('dateRange')
  dateRangeComponent: MatSelect

  private chart: am4charts.XYChart;

  constructor(
    private runsQuery: RunsQuery,
    private runsService: RunsService) {
  }

  ngOnInit(): void {
    this.runsService.load();

    this.dateRanges.push({
      label: "All",
      startDate: new Date(2000, 0, 1).getTime() / 1000
    });

    const last5Years = new Date();
    last5Years.setFullYear(last5Years.getFullYear() - 5);
    this.dateRanges.push({
      label: "Last 5 Years",
      startDate: last5Years.getTime() / 1000
    });

    const last6Months = new Date();
    last6Months.setMonth(last6Months.getMonth() - 6);
    this.dateRanges.push({
      label: "Last 6 Months",
      startDate: last6Months.getTime() / 1000
    });

    this.selectedDateRange = this.dateRanges[this.dateRanges.length - 1];
  }

  ngAfterViewInit(): void {

    const startDateObs = this.dateRangeComponent.selectionChange.pipe(
      map((event: MatSelectChange) => (event.value as DateRange).startDate),
      startWith(this.selectedDateRange.startDate)
    )

    // am4core.useTheme(am4themes_animated);
    this.runsQuery.allRunsSorted$.pipe(
      untilDestroyed(this),
      filter(r => r.length > 0),
      switchMap(runs => startDateObs.pipe(map(startDate => ({runs, startDate}))))
    ).subscribe(({runs, startDate}) => {
      // console.log('got runs', runs, 'and start at', startDate);

      if (this.chart != null) {
        this.chart.dispose();
      }

      // Create chart instance
      this.chart = am4core.create('chartdiv', am4charts.XYChart);

      this.chart.data = runs
        .filter(run => run.date > startDate)
        .map(run => {
          return {
            date: run.date * 1000, length: run.length / 1000, timePerKm: run.timeUsed / run.length * 1000
          };
        })
        .reverse();


      const dateAxis = this.chart.xAxes.push(new am4charts.DateAxis());
      // dateAxis.renderer.minGridDistance = 50;

      // Distance on bar chart
      const distanceValueAxis = this.chart.yAxes.push(new am4charts.ValueAxis());
      distanceValueAxis.title.text = 'Distance (km)';
      distanceValueAxis.renderer.tooltip.disabled = true;
      distanceValueAxis.renderer.grid.template.disabled = true;
      distanceValueAxis.renderer.grid.template.location = 0;

      const distanceSeries = this.chart.series.push(new am4charts.ColumnSeries());
      distanceSeries.dataFields.valueY = "length";
      distanceSeries.dataFields.dateX = "date";
      distanceSeries.xAxis = dateAxis;
      distanceSeries.yAxis = distanceValueAxis;
      distanceSeries.tooltip.disabled = true;

      // time per km on line chart
      const perKmValueAxis = this.chart.yAxes.push(new am4charts.DurationAxis());
      perKmValueAxis.title.text = 'Time per km';
      perKmValueAxis.renderer.opposite = true;
      perKmValueAxis.syncWithAxis = distanceValueAxis;
      perKmValueAxis.baseUnit = 'second';
      perKmValueAxis.renderer.tooltip.disabled = true;
      perKmValueAxis.renderer.grid.template.disabled = true;
      perKmValueAxis.renderer.grid.template.location = 0;

      const perKmSeries = this.chart.series.push(new am4charts.LineSeries());
      perKmSeries.dataFields.valueY = "timePerKm";
      perKmSeries.dataFields.dateX = "date";
      perKmSeries.xAxis = dateAxis;
      perKmSeries.yAxis = perKmValueAxis;
      perKmSeries.strokeWidth = 2;
      perKmSeries.tooltipText = "{dateX.formatDate('dd.MM.yyyy HH:mm')}\nper km: [bold]{valueY.formatDuration()}[/]\nDistance: [bold]{length}[/]";

      // Add cursor
      const cursor = new am4charts.XYCursor();
      cursor.xAxis = dateAxis;
      cursor.behavior = 'panX';
      cursor.lineY.disabled = true;
      cursor.fullWidthLineX = true;
      cursor.lineX.strokeWidth = 0;
      cursor.lineX.fill = am4core.color("#8F3985");
      cursor.lineX.fillOpacity = 0.1;
      this.chart.cursor = cursor;

      const scrollbar = new am4charts.XYChartScrollbar();
      scrollbar.series.push(distanceSeries);
      scrollbar.series.push(perKmSeries);
      // calculate the position of 2 weeks ago in a relative number (0-1)
      const twoWeeksInMs = 2 * 7 * 24 * 60 * 60 * 1000;
      const totalMs = Date.now() - this.chart.data[0].date;
      const twoWeeksRelative = twoWeeksInMs / totalMs;
      scrollbar.start = 1 - twoWeeksRelative;
      // console.log(twoWeeksInMs, Date.now(), this.chart.data[0].date, totalMs, twoWeeksInMs, twoWeeksRelative);
      this.chart.scrollbarX = scrollbar;
    });

  }

  ngOnDestroy(): void {
    if (this.chart != null) {
      this.chart.dispose();
    }
  }


}
