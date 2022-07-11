import 'package:app_flowy/workspace/application/grid/prelude.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/cell/cell_builder.dart';
import 'package:app_flowy/startup/startup.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/cell/select_option_cell/select_option_editor.dart';
import 'package:flowy_infra/theme.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flowy_sdk/protobuf/flowy-grid/select_option.pb.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import '../cell_builder.dart';
import 'extension.dart';
import 'select_option_cell.dart';

class ChecklistSelectCell extends GridCellWidget {
  final GridCellContextBuilder cellContextBuilder;
  late final SelectOptionCellStyle? cellStyle;

  ChecklistSelectCell({
    required this.cellContextBuilder,
    GridCellStyle? style,
    Key? key,
  }) : super(key: key) {
    if (style != null) {
      cellStyle = (style as SelectOptionCellStyle);
    } else {
      cellStyle = null;
    }
  }

  @override
  State<ChecklistSelectCell> createState() => _ChecklistSelectCellState();
}

class _ChecklistSelectCellState extends State<ChecklistSelectCell> {
  late SelectOptionCellBloc _cellBloc;

  @override
  void initState() {
    final cellContext = widget.cellContextBuilder.build() as GridSelectOptionCellContext;
    _cellBloc = getIt<SelectOptionCellBloc>(param1: cellContext)..add(const SelectOptionCellEvent.initial());
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider.value(
      value: _cellBloc,
      child: BlocBuilder<SelectOptionCellBloc, SelectOptionCellState>(
        builder: (context, state) {
          return _SelectOptionCell(
              selectOptions: state.selectedOptions,
              cellStyle: widget.cellStyle,
              onFocus: (value) => widget.onCellEditing.value = value,
              cellContextBuilder: widget.cellContextBuilder);
        },
      ),
    );
  }

  @override
  Future<void> dispose() async {
    _cellBloc.close();
    super.dispose();
  }
}

class _SelectOptionCell extends StatelessWidget {
  final List<SelectOption> selectOptions;
  final void Function(bool) onFocus;
  final SelectOptionCellStyle? cellStyle;
  final GridCellContextBuilder cellContextBuilder;
  const _SelectOptionCell({
    required this.selectOptions,
    required this.onFocus,
    required this.cellStyle,
    required this.cellContextBuilder,
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = context.watch<AppTheme>();
    final Widget child;
    if (selectOptions.isEmpty && cellStyle != null) {
      child = Align(
        alignment: Alignment.centerLeft,
        child: FlowyText.medium(cellStyle!.placeholder, fontSize: 14, color: theme.shader3),
      );
    } else {
      final tags = selectOptions
          .map(
            (option) => SelectOptionTag.fromSelectOption(
              context: context,
              option: option,
            ),
          )
          .toList();
      child = Align(
        alignment: Alignment.centerLeft,
        child: Wrap(children: tags, spacing: 4, runSpacing: 2),
      );
    }

    return Stack(
      alignment: AlignmentDirectional.center,
      fit: StackFit.expand,
      children: [
        child,
        InkWell(
          onTap: () {
            onFocus(true);
            final cellContext = cellContextBuilder.build() as GridSelectOptionCellContext;
            SelectOptionCellEditor.show(context, cellContext, () => onFocus(false));
          },
        ),
      ],
    );
  }
}
